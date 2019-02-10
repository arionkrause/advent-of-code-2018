pub fn solve(input: &str) -> usize {
    let target_sequence: Vec<usize> = input.replace("\r\n", "")
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

    let mut scoreboard = Vec::new();
    scoreboard.push(3);
    scoreboard.push(7);

    let mut elf_1_index = 0;
    let mut elf_2_index = 1;

    loop {
        let new_recipes = create_recipes(scoreboard[elf_1_index], scoreboard[elf_2_index]);

        for recipe in new_recipes.into_iter() {
            scoreboard.push(recipe);

            if scoreboard.len() >= target_sequence.len() {
                if scoreboard[scoreboard.len() - target_sequence.len()..scoreboard.len()]
                        == target_sequence[..] {
                    return scoreboard.len() - target_sequence.len()
                }
            }
        }

        elf_1_index = (elf_1_index + 1 + scoreboard[elf_1_index]) % scoreboard.len();
        elf_2_index = (elf_2_index + 1 + scoreboard[elf_2_index]) % scoreboard.len();
    };
}

fn create_recipes(recipe1: usize, recipe2: usize) -> Vec<usize> {
    let mut new_recipes = Vec::new();
    let sum = recipe1 + recipe2;
    let sum_string = (sum as u32).to_string();
    sum_string.chars().for_each(|d| new_recipes.push(d.to_digit(10).unwrap() as usize));
    new_recipes
}

#[cfg(test)]
mod test;
