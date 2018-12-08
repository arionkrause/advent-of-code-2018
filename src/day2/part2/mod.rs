pub fn solve(input: &str) -> String {
    for (index, line) in input.lines().enumerate() {
        for line_other in input.lines().skip(index + 1) {
            let same_characters = line.chars()
                .zip(line_other.chars())
                .filter(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect::<String>();

            if same_characters.len() == line.len() - 1 {
                return same_characters;
            }
        }
    }

    panic!("Should not have reached here.");
}

#[cfg(test)]
mod test;
