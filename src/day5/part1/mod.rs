pub fn solve(input: &str) -> usize {
    let polymer = input.replace("\n", "").replace("\r", "").as_bytes().to_owned();
    let mut result: Vec<u8> = Vec::new();

    for letter in polymer {
        if result.last().map(|&v| v ^ letter == 32).unwrap_or(false) {
            result.pop();
        } else {
            result.push(letter);
        }
    }

    result.len()
}

#[cfg(test)]
mod test;
