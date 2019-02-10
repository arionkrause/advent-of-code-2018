use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Group {
    name: String,
    group_type: GroupType,
    amount_units: usize,
    unit_hitpoints: usize,
    amount_damage: usize,
    damage_type: DamageType,
    initiative: usize,
    weaknesses: Vec<DamageType>,
    immunities: Vec<DamageType>,
    alive: bool,
}

impl Group {
    fn effective_power(&self) -> usize {
        self.amount_units * self.amount_damage
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum GroupType {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Eq, PartialEq)]
enum DamageType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

pub fn solve(input: &str) -> usize {
    let mut groups = decode_input(&input);

    loop {
        let targeting = target(&groups);
        attack(&mut groups, &targeting);

        if combat_has_ended(&groups) {
            return amount_units_remaining(&groups);
        }
    }
}

fn attack(groups: &mut Vec<Group>, targeting: &HashMap<usize, usize>) {
    let mut indices_ordered_by_decreasing_initiative = groups.iter()
            .enumerate()
            .filter(|(index, _)| targeting.get(index).is_some())
            .map(|(index, group)| (index, group.initiative))
            .collect::<Vec<(usize, usize)>>();

    indices_ordered_by_decreasing_initiative.sort_by(|(_, a_initiative), (_, b_initiative)| {
        a_initiative.cmp(&b_initiative).reverse()
    });

    let indices_ordered_by_decreasing_initiative = indices_ordered_by_decreasing_initiative.into_iter()
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

    for &attacker in indices_ordered_by_decreasing_initiative.iter() {
        let &target = targeting.get(&attacker).unwrap();
        let maximum_amount_killed_units = calculate_damage(&groups[attacker], &groups[target]).unwrap() / groups[target].unit_hitpoints;

        if maximum_amount_killed_units > groups[target].amount_units {
            groups[target].amount_units = 0;
            groups[target].alive = false;
        } else {
            groups[target].amount_units -= maximum_amount_killed_units;
        }
    }
}

fn target(groups: &Vec<Group>) -> HashMap<usize, usize> {
    let mut indices_ordered_by_decreasing_effective_power = groups.iter()
            .enumerate()
            .filter(|(_, group)| group.alive)
            .map(|(index, group)| (index, group.effective_power()))
            .collect::<Vec<(usize, usize)>>();

    indices_ordered_by_decreasing_effective_power.sort_by(|(_, a_effective_power), (_, b_effective_power)| {
        a_effective_power.cmp(&b_effective_power).reverse()
    });

    let indices_ordered_by_decreasing_effective_power = indices_ordered_by_decreasing_effective_power.into_iter()
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

    let mut targeting: HashMap<usize, usize> = HashMap::new();

    for index in indices_ordered_by_decreasing_effective_power.into_iter() {
        let mut targeted_enemy: Option<usize> = None;
        let mut most_damage = None;

        for (index_enemy, group_enemy) in groups.iter()
                .enumerate()
                .filter(|(_, enemy_group)| enemy_group.alive && enemy_group.group_type != groups[index].group_type) {
            if targeting.iter().any(|(_, i)| i == &index_enemy) {
                continue;
            }

            let calculated_damage = calculate_damage(&groups[index], &groups[index_enemy]);

            if calculated_damage.is_none() {
                continue;
            }

            if targeted_enemy.is_none()
                    || calculated_damage > most_damage.unwrap()
                    || (calculated_damage == most_damage.unwrap()
                    && group_enemy.effective_power() > groups[targeted_enemy.unwrap()].effective_power())
                    || (calculated_damage == most_damage.unwrap()
                    && group_enemy.effective_power() == groups[targeted_enemy.unwrap()].effective_power()
                    && group_enemy.initiative > groups[targeted_enemy.unwrap()].initiative) {
                targeted_enemy = Some(index_enemy);
                most_damage = Some(calculated_damage);
            }
        }

        if targeted_enemy.is_some() {
            targeting.insert(index, targeted_enemy.unwrap());
        }
    }

    targeting
}

fn calculate_damage(attacker: &Group, target: &Group) -> Option<usize> {
    if target.immunities.contains(&attacker.damage_type) {
        return None;
    }

    if target.weaknesses.contains(&attacker.damage_type) {
        return Some(attacker.effective_power() * 2);
    }

    Some(attacker.effective_power())
}

fn combat_has_ended(groups: &Vec<Group>) -> bool {
    !groups.iter().any(|g| g.group_type == GroupType::ImmuneSystem && g.amount_units > 0)
            || !groups.iter().any(|g| g.group_type == GroupType::Infection && g.amount_units > 0)
}

fn amount_units_remaining(groups: &Vec<Group>) -> usize {
    groups.iter().map(|g| g.amount_units).sum()
}

fn decode_input(input: &str) -> Vec<Group> {
    let immune_system_data = &input[input.find("Immune System:\r\n").unwrap() + 16..input.find("\r\n\r\n").unwrap() + 2];
    let infection_data = &input[input.find("Infection:\r\n").unwrap() + 12..];
    let mut groups = decode_groups(&immune_system_data, &GroupType::ImmuneSystem);
    groups.extend(decode_groups(&infection_data, &GroupType::Infection));
    groups
}

fn decode_groups(input: &str, group_type: &GroupType) -> Vec<Group> {
    let re_main = regex::Regex::new(r"^(?P<amount_units>\d+) units each with (?P<unit_hitpoints>\d+) hit points .*?with an attack that does (?P<amount_damage>\d+) (?P<damage_type>[[:alpha:]]+) damage at initiative (?P<initiative>\d+)$").unwrap();
    let re_weaknesses = Regex::new(r"weak to (?P<weaknesses>.+?)[;)]").unwrap();
    let re_immunities = Regex::new(r"immune to (?P<immunities>.+?)[;)]").unwrap();
    let mut groups: Vec<Group> = Vec::new();

    for line in input.lines() {
        let mut next_immune_system_name = String::from("Immune System group ");
        next_immune_system_name.push_str(&(groups.iter().filter(|g| g.group_type == GroupType::ImmuneSystem).count() + 1).to_string());
        let mut next_infection_name = String::from("Infection group ");
        next_infection_name.push_str(&(groups.iter().filter(|&g| g.group_type == GroupType::Infection).count() + 1).to_string());
        groups.push(decode_group(&line, &group_type, &re_main, &re_weaknesses, &re_immunities, next_immune_system_name, next_infection_name));
    }

    groups
}

fn decode_group(input: &str, group_type: &GroupType, re_main: &Regex, re_weaknesses: &Regex, re_immunities: &Regex, next_immune_system_name: String, next_infection_name: String) -> Group {
    let captures = re_main.captures(&input).unwrap();
    let amount_units = captures.name("amount_units").unwrap().as_str().parse::<usize>().unwrap();
    let unit_hitpoints = captures.name("unit_hitpoints").unwrap().as_str().parse::<usize>().unwrap();
    let amount_damage = captures.name("amount_damage").unwrap().as_str().parse::<usize>().unwrap();
    let damage_type_in_word = captures.name("damage_type").unwrap().as_str();
    let initiative = captures.name("initiative").unwrap().as_str().parse::<usize>().unwrap();
    let damage_type = decode_damage_type(&damage_type_in_word);
    let mut weaknesses = Vec::new();
    let mut immunities = Vec::new();

    if let Some(weaknesses_captures) = re_weaknesses.captures(&input) {
        for weaknesses_in_words in weaknesses_captures.name("weaknesses").unwrap().as_str().split(", ") {
            weaknesses.push(decode_damage_type(&weaknesses_in_words));
        }
    }

    if let Some(immunities_captures) = re_immunities.captures(&input) {
        for immunities_in_words in immunities_captures.name("immunities").unwrap().as_str().split(", ") {
            immunities.push(decode_damage_type(&immunities_in_words));
        }
    }

    let name = match group_type {
        GroupType::ImmuneSystem => next_immune_system_name,
        GroupType::Infection => next_infection_name,
    };

    Group {
        name,
        group_type: group_type.clone(),
        amount_units,
        unit_hitpoints,
        amount_damage,
        damage_type,
        initiative,
        weaknesses,
        immunities,
        alive: true,
    }
}

fn decode_damage_type(damage_type_in_word: &str) -> DamageType {
    match damage_type_in_word {
        "bludgeoning" => DamageType::Bludgeoning,
        "cold" => DamageType::Cold,
        "fire" => DamageType::Fire,
        "radiation" => DamageType::Radiation,
        "slashing" => DamageType::Slashing,
        _ => panic!(),
    }
}

#[cfg(test)]
mod test;
