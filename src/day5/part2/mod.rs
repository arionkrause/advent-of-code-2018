pub fn solve(input: &str) -> usize {
    let polymer = input.replace("\n", "").replace("\r", "").as_bytes().to_owned();
    let reduced = get_reacted_polymer(polymer.iter().cloned());

    (b'a'..=b'z')
        .map(|c| get_reacted_polymer(reduced.iter().cloned().filter(|&b| b | 32 != c)).len())
        .min()
        .unwrap()
}

fn get_reacted_polymer(polymer: impl Iterator<Item = u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();

    for letter in polymer {
        if result.last().map(|&v| v ^ letter == 32).unwrap_or(false) {
            result.pop();
        } else {
            result.push(letter);
        }
    }

    result
}

#[cfg(test)]
mod test;
