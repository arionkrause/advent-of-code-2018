pub fn solve(input: &str) -> isize {
    let mut current_frequency: isize = 0;

    for line in input.lines() {
        let change: isize = line.parse().unwrap();
        current_frequency += change;
    }

    current_frequency
}

#[cfg(test)]
mod test;
