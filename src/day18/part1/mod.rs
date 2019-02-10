type Grid = Vec<Vec<char>>;

pub fn solve(input: &str, minutes: usize) -> usize {
    let mut grid = decode_input(&input);

    for _ in 1..=minutes {
        let mut new_grid = Vec::new();

        for (x, row) in grid.iter().enumerate() {
            let mut new_row = Vec::new();

            for (y, &tile) in row.iter().enumerate() {
                if tile == '.' {
                    new_row.push(if amount_adjacent_tiles_of(&grid, x, y, '|') >= 3 {
                        '|'
                    } else {
                        '.'
                    });
                } else if tile == '|' {
                    new_row.push(if amount_adjacent_tiles_of(&grid, x, y, '#') >= 3 {
                        '#'
                    } else {
                        '|'
                    });
                } else if tile == '#' {
                    new_row.push(if amount_adjacent_tiles_of(&grid, x, y, '#') >= 1
                            && amount_adjacent_tiles_of(&grid, x, y, '|') >= 1 {
                        '#'
                    } else {
                        '.'
                    });
                }
            }

            new_grid.push(new_row);
        }

        grid = new_grid;
    }

    print_grid(&grid);
    let mut total_wooded = 0;
    let mut total_lumberyeards = 0;

    for row in grid.iter() {
        for &tile in row.iter() {
            if tile == '|' {
                total_wooded += 1;
            } else if tile == '#' {
                total_lumberyeards += 1;
            }
        }
    }

    total_wooded * total_lumberyeards
}

fn amount_adjacent_tiles_of(grid: &Grid, x: usize, y: usize, tile_type: char) -> usize {
    let mut total = 0;

    if x > 0 && y > 0 && grid[x - 1][y - 1] == tile_type {
        total += 1;
    }

    if x > 0 && grid[x - 1][y] == tile_type {
        total += 1;
    }

    if x > 0 && y < grid[0].len() - 1 && grid[x - 1][y + 1] == tile_type {
        total += 1;
    }


    if y > 0 && grid[x][y - 1] == tile_type {
        total += 1;
    }

    if y < grid[0].len() - 1 && grid[x][y + 1] == tile_type {
        total += 1;
    }


    if x < grid.len() - 1 && y > 0 && grid[x + 1][y - 1] == tile_type {
        total += 1;
    }

    if x < grid.len() - 1 && grid[x + 1][y] == tile_type {
        total += 1;
    }

    if x < grid.len() - 1 && y < grid[0].len() - 1 && grid[x + 1][y + 1] == tile_type {
        total += 1;
    }

    total
}

fn print_grid(grid: &Grid) {
    for row in grid.iter() {
        for tile in row.iter() {
            print!("{}", tile);
        }

        println!();
    }

    println!();
}

fn decode_input(input: &str) -> Grid {
    let mut grid = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for tile in line.chars() {
            row.push(tile);
        }

        grid.push(row);
    }

    grid
}

#[cfg(test)]
mod test;
