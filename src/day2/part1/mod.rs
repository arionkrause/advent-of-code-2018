use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut total_letters_appear_twice: usize = 0;
    let mut total_letters_appear_thrice: usize = 0;

    for line in input.lines() {
        let mut seen_letters = HashMap::new();

        for letter in line.chars() {
            seen_letters.entry(letter)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);
        }

        if seen_letters.values().any(|&e| e == 2) {
            total_letters_appear_twice += 1;
        }

        if seen_letters.values().any(|&e| e == 3) {
            total_letters_appear_thrice += 1;
        }
    }

    total_letters_appear_twice * total_letters_appear_thrice
}

#[cfg(test)]
mod test;
