pub fn solve(input: &str) -> usize {
    let amount_recipes = input.replace("\r\n", "").parse::<usize>().unwrap();
    let mut scoreboard = Vec::new();
    scoreboard.push(3);
    scoreboard.push(7);

    let mut elf_1_index = 0;
    let mut elf_2_index = 1;

    for _ in 0..amount_recipes + 10 {
        let new_recipes = create_recipes(scoreboard[elf_1_index], scoreboard[elf_2_index]);

        for recipe in new_recipes.into_iter() {
            scoreboard.push(recipe);
        }

        elf_1_index = (elf_1_index + 1 + scoreboard[elf_1_index]) % scoreboard.len();
        elf_2_index = (elf_2_index + 1 + scoreboard[elf_2_index]) % scoreboard.len();
    };

    let mut result = String::new();

    for recipe in scoreboard[amount_recipes..amount_recipes + 10].iter() {
        result.push_str(&(*recipe as u32).to_string());
    }

    result.parse::<usize>().unwrap()
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
