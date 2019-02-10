use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::Ordering;

type Grid = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
}

#[derive(Debug)]
struct Unit {
    id: usize,
    unit_type: UnitType,
    position: Position,
    hitpoints: i16,
    alive: bool,
    last_played: Option<usize>,
}

#[derive(Debug, PartialEq)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Clone, Debug, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        if self.x == other.x {
            return self.y.cmp(&other.y);
        }

        return self.x.cmp(&other.x);
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.y == other.y
                && self.x == other.x
    }
}

pub fn solve(input: &str) -> usize {
    let (grid, units) = decode_game(&input);
    let mut round = 1;
    eprintln!("Initially:");
    eprint_grid(&grid, &units);
    eprint_units(&units);

    loop {
        eprintln!("-------------------------------------------------------------------");
        eprintln!("Round {:?} started.", round);

        while let Some(unit) = get_next_playing_unit(round, &units) {
            eprintln!();
            eprintln!("{:?}", unit.borrow());
            if can_attack(&units, &unit) {
                attack(&unit, &units);
            } else {
                move_unit(&grid, &units, &unit);

                if can_attack(&units, &unit) {
                    attack(&unit, &units);
                }
            };

            unit.borrow_mut().last_played = Some(round);

            if combat_has_ended(&units) {
                if get_next_playing_unit(round, &units).is_some() {
                    return (round - 1) * get_sum_hitpoints_alive_units(&units) as usize;
                } else {
                    return round * get_sum_hitpoints_alive_units(&units) as usize;
                }
            }
        };

        eprintln!();
        eprintln!("After {} round{}:", round, if round > 1 { "s" } else { "" });
        eprint_grid(&grid, &units);
        eprint_units(&units);
        eprintln!("-------------------------------------------------------------------");

        round += 1;
    }
}

fn decode_game(input: &str) -> (Grid, Vec<Rc<RefCell<Unit>>>) {
    let mut grid = Vec::new();
    let mut units = Vec::new();
    let mut unit_id = 1;

    for (x, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (y, tile) in line.chars().enumerate() {
            match tile {
                '#' => row.push(Tile::Wall),
                '.' => row.push(Tile::Open),
                ut @ 'E' | ut @ 'G' => {
                    let unit_type = if ut == 'E' {
                        UnitType::Elf
                    } else {
                        UnitType::Goblin
                    };

                    let unit = Unit {
                        id: unit_id,
                        unit_type,
                        position: Position {
                            x,
                            y,
                        },
                        hitpoints: 200,
                        alive: true,
                        last_played: None,
                    };

                    units.push(Rc::new(RefCell::new(unit)));
                    unit_id += 1;
                    row.push(Tile::Open);
                },
                _ => panic!()
            }
        }

        grid.push(row);
    }

    (grid, units)
}

fn get_next_playing_unit(round: usize, units: &Vec<Rc<RefCell<Unit>>>) -> Option<&Rc<RefCell<Unit>>> {
    let mut available_units: Vec<&Rc<RefCell<Unit>>> = units.iter()
            .filter(|&u| {
                u.borrow().alive
                        && (u.borrow().last_played.is_none() || u.borrow().last_played.unwrap() < round)
            })
            .collect();

    if available_units.is_empty() {
        return None;
    }

    available_units.sort_by(|a, b| {
        a.borrow().position.cmp(&b.borrow().position)
    });

    Some(available_units[0])
}

fn can_attack(units: &Vec<Rc<RefCell<Unit>>>, unit: &Rc<RefCell<Unit>>) -> bool {
    for other_unit in units.iter() {
        if !other_unit.borrow().alive
                || other_unit.borrow().id == unit.borrow().id
                || other_unit.borrow().unit_type == unit.borrow().unit_type {
            continue;
        }

        if (other_unit.borrow().position.x as isize - unit.borrow().position.x as isize).abs()
                + (other_unit.borrow().position.y as isize - unit.borrow().position.y as isize).abs() == 1 {
            return true;
        }
    }

    false
}

fn attack(unit: &Rc<RefCell<Unit>>, units: &Vec<Rc<RefCell<Unit>>>) {
    eprintln!("Unit {} is attacking.", unit.borrow().id);
    let mut attackable_units = Vec::new();

    for other_unit in units.iter() {
        if !other_unit.borrow().alive
                || other_unit.borrow().id == unit.borrow().id
                || other_unit.borrow().unit_type == unit.borrow().unit_type {
            continue;
        }

        if (other_unit.borrow().position.x as isize - unit.borrow().position.x as isize).abs()
                + (other_unit.borrow().position.y as isize - unit.borrow().position.y as isize).abs() == 1 {
            attackable_units.push(other_unit);
        }
    }

    if attackable_units.len() == 1 {
        let attacked_unit = attackable_units[0];
        attacked_unit.borrow_mut().hitpoints -= 3;
        eprintln!("Unit {} attacked Unit {}, whose HP is now {}.", unit.borrow().id, attacked_unit.borrow().id, attacked_unit.borrow().hitpoints);

        if attacked_unit.borrow_mut().hitpoints < 0 {
            attacked_unit.borrow_mut().alive = false;
            eprintln!("Unit {} died.", attacked_unit.borrow().id);
        }

        return;
    }

    attackable_units.sort_by(|&a, &b| a.borrow().hitpoints.cmp(&b.borrow().hitpoints));
    let lowest_hitpoints = attackable_units[0].borrow().hitpoints;
    let mut attackable_units_with_same_lowest_hitpoints = Vec::new();

    for attackable_unit in attackable_units {
        if attackable_unit.borrow().hitpoints == lowest_hitpoints {
            attackable_units_with_same_lowest_hitpoints.push(attackable_unit);
        }
    }

    if attackable_units_with_same_lowest_hitpoints.len() > 1 {
        attackable_units_with_same_lowest_hitpoints.sort_by(|a, b| a.borrow().position.cmp(&b.borrow().position));
    }

    let attacked_unit = &attackable_units_with_same_lowest_hitpoints[0];
    attacked_unit.borrow_mut().hitpoints -= 3;
    eprintln!("Unit {} attacked Unit {}, whose HP is now {}.", unit.borrow().id, attacked_unit.borrow().id, attacked_unit.borrow().hitpoints);

    if attacked_unit.borrow_mut().hitpoints < 0 {
        attacked_unit.borrow_mut().alive = false;
        eprintln!("Unit {} died.", attacked_unit.borrow().id);
    }
}

fn move_unit(grid: &Grid, units: &Vec<Rc<RefCell<Unit>>>, unit: &Rc<RefCell<Unit>>) {
    if let Some(next_position) = get_next_position(&grid, &units, &unit) {
        unit.borrow_mut().position = next_position;
        eprintln!("Unit {} moved to {}, {}", unit.borrow().id, unit.borrow().position.x, unit.borrow().position.y);
    }
}

fn get_next_position<'a>(grid: &Grid, units: &'a Vec<Rc<RefCell<Unit>>>, unit: &Rc<RefCell<Unit>>) -> Option<Position> {
    let mut queue = Vec::new();
    let mut visited = Vec::new();
    try_add_neighbor(&grid, &units, &mut visited, &mut queue, 1, Position { x: unit.borrow().position.x - 1, y: unit.borrow().position.y }, &Position { x: unit.borrow().position.x - 1, y: unit.borrow().position.y }, false, &unit.borrow().unit_type);
    try_add_neighbor(&grid, &units, &mut visited, &mut queue, 1, Position { x: unit.borrow().position.x, y: unit.borrow().position.y + 1 }, &Position { x: unit.borrow().position.x, y: unit.borrow().position.y + 1 }, false, &unit.borrow().unit_type);
    try_add_neighbor(&grid, &units, &mut visited, &mut queue, 1, Position { x: unit.borrow().position.x + 1, y: unit.borrow().position.y }, &Position { x: unit.borrow().position.x + 1, y: unit.borrow().position.y }, false, &unit.borrow().unit_type);
    try_add_neighbor(&grid, &units, &mut visited, &mut queue, 1, Position { x: unit.borrow().position.x, y: unit.borrow().position.y - 1 }, &Position { x: unit.borrow().position.x, y: unit.borrow().position.y - 1 }, false, &unit.borrow().unit_type);

    loop {
        if queue.is_empty() {
            return None;
        }

        queue.sort_by(|(position_a, distance_a, first_move_a), (position_b, distance_b, first_move_b)| {
            if distance_a == distance_b {
                if position_a == position_b {
                    first_move_a.cmp(first_move_b)
                } else {
                    position_a.cmp(position_b)
                }
            } else {
                distance_a.cmp(distance_b)
            }
        });

        let (position, distance, first_position) = queue.remove(0);

        for other_unit in units.iter() {
            if other_unit.borrow().alive
                    && other_unit.borrow().id != unit.borrow().id
                    && other_unit.borrow().unit_type != unit.borrow().unit_type
                    && other_unit.borrow().position == position {
                return Some(first_position);
            }
        }

        try_add_neighbor(&grid, &units, &mut visited, &mut queue, distance + 1, Position { x: position.x - 1, y: position.y }, &first_position, true, &unit.borrow().unit_type);
        try_add_neighbor(&grid, &units, &mut visited, &mut queue, distance + 1, Position { x: position.x, y: position.y + 1 }, &first_position, true, &unit.borrow().unit_type);
        try_add_neighbor(&grid, &units, &mut visited, &mut queue, distance + 1, Position { x: position.x + 1, y: position.y }, &first_position, true, &unit.borrow().unit_type);
        try_add_neighbor(&grid, &units, &mut visited, &mut queue, distance + 1, Position { x: position.x, y: position.y - 1 }, &first_position, true, &unit.borrow().unit_type);
    }
}

fn try_add_neighbor(
    grid: &Grid,
    units: &Vec<Rc<RefCell<Unit>>>,
    visited: &mut Vec<Position>,
    queue: &mut Vec<(Position, u8, Position)>,
    distance: u8,
    position: Position,
    first_position: &Position,
    add_to_visited: bool,
    friendly_unit_type: &UnitType
) {
    if visited.contains(&position) {
        return;
    }

    if grid[position.x][position.y] == Tile::Open
            && !units.iter().any(|other_unit| other_unit.borrow().alive
                && other_unit.borrow().position == position
                && &other_unit.borrow().unit_type == friendly_unit_type) {
        queue.push((position.clone(), distance, first_position.clone()));

        if add_to_visited {
            visited.push(position);
        }
    }
}

fn combat_has_ended(units: &Vec<Rc<RefCell<Unit>>>) -> bool {
    let remaining_elves = units.iter()
            .filter(|&unit| unit.borrow().unit_type == UnitType::Elf
                    && unit.borrow().alive)
            .count();

    if remaining_elves == 0 {
        return true;
    }

    let remaining_goblins = units.iter()
            .filter(|&unit| unit.borrow().unit_type == UnitType::Goblin
                    && unit.borrow().alive)
            .count();

    remaining_goblins == 0
}

fn get_sum_hitpoints_alive_units(units: &Vec<Rc<RefCell<Unit>>>) -> i16 {
    units.iter()
            .filter(|&u| u.borrow().alive)
            .map(|u| u.borrow().hitpoints)
            .sum()
}

fn eprint_grid(grid: &Grid, units: &Vec<Rc<RefCell<Unit>>>) {
    for (x, row) in grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            let mut there_is_an_unit_in_here = false;

            for unit in units.iter() {
                if unit.borrow().alive
                        && unit.borrow().position.x == x
                        && unit.borrow().position.y == y {
                    eprint!("{}", match unit.borrow().unit_type {
                        UnitType::Elf => 'E',
                        UnitType::Goblin => 'G',
                    });

                    there_is_an_unit_in_here = true;
                    break;
                }
            }

            if !there_is_an_unit_in_here {
                eprint!("{}", match tile {
                    Tile::Open => '.',
                    Tile::Wall => '#',
                });
            }
        }

        eprintln!();
    }

    eprintln!();
}

fn eprint_units(units: &Vec<Rc<RefCell<Unit>>>) {
    let mut sorted_units = units.clone();
    sorted_units.sort_by(|a, b| a.borrow().position.cmp(&b.borrow().position));

    for unit in sorted_units.iter() {
        eprintln!("{:?}", unit.borrow());
    }
}

#[cfg(test)]
mod test;
