use regex::Regex;
use std::collections::HashMap;

pub fn solve(input: &str, amount_generations: usize) -> isize {
    let repetitions_needed_to_declare_convergence = 12;
    let mut state = String::from("...");
    state.push_str(&get_initial_state(&input));
    state.push_str("...");
    let rules = get_rules_plant(&input);
    let mut last = 0;
    let mut diffs: HashMap<isize, u32> = HashMap::new();

    for generation in 1..=amount_generations {
        let mut new_state = String::from("...");

        for i in 2..state.len() - 2 {
            let combination = &state[i - 2..=i + 2];

            match rules.get(combination) {
                Some(content) => {
                    new_state.push_str(content);
                },
                None => new_state.push_str("."),
            }
        }

        new_state.push_str("...");
        state = new_state;

        let score = state.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(|(i, _)| i as isize - (3 + generation as isize))
                .sum::<isize>();

        let entry = diffs.entry(score as isize - last as isize).or_insert(0);

        if *entry > repetitions_needed_to_declare_convergence {
            return (amount_generations - generation) as isize * (score - last) + score;
        } else {
            *entry += 1;
        }

        last = score;
    }

    last
}

fn get_initial_state(input: &str) -> String {
    let re = Regex::new(r"^initial state: ([\\.#]+)$").unwrap();

    for line in input.lines() {
        let captures = re.captures(line);

        if captures.is_some() {
            let captures = captures.unwrap();
            return captures.get(1).unwrap().as_str().to_owned();
        }
    }

    panic!()
}

fn get_rules_plant(input: &str) -> HashMap<String, String> {
    let re = Regex::new(r"^([\\.#]+?) => ([\\.#])$").unwrap();
    let mut rules = HashMap::new();

    for line in input.lines() {
        let captures = re.captures(line);

        if captures.is_some() {
            let captures = captures.unwrap();
            let combination: String = captures.get(1).unwrap().as_str().to_owned();
            let content = captures.get(2).unwrap().as_str().to_owned();

            if content == "#" {
                rules.insert(combination, content);
            }
        }
    }

    rules
}
