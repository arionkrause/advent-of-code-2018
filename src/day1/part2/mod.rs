use std::collections::HashSet;

pub fn solve(input: &str) -> isize {
    let mut current_frequency: isize = 0;
    let mut seen_frequencies = HashSet::new();
    seen_frequencies.insert(current_frequency);

    loop {
        for line in input.lines() {
            let change: isize = line.parse().unwrap();
            current_frequency += change;

            if seen_frequencies.contains(&current_frequency) {
                return current_frequency;
            }

            seen_frequencies.insert(current_frequency);
        }
    }
}

#[cfg(test)]
mod test;
