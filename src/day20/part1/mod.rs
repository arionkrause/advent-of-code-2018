use std::collections::HashMap;

type Grid = Vec<Vec<char>>;

static GRID_SIZE: usize = 2000;
static STARTING_POSITION: (usize, usize) = (GRID_SIZE / 2, GRID_SIZE / 2);

pub fn solve(input: &str) -> usize {
    let grid = decode_grid(&input);
    eprint_grid(&grid);
    get_maximum_distance(&grid)
}

fn decode_grid(input: &str) -> Grid {
    let mut grid = vec![vec!['#'; GRID_SIZE]; GRID_SIZE];
    let mut positions = Vec::new();
    let mut current_position = STARTING_POSITION;
    let mut position_checkpoint = current_position.clone();
    grid[current_position.0][current_position.1] = 'X';

    for tile in input.chars() {
        match tile {
            'N' => {
                current_position = (current_position.0 - 2, current_position.1);
                grid[current_position.0][current_position.1] = '.';
                grid[current_position.0 + 1][current_position.1] = '-';
            },
            'E' => {
                current_position = (current_position.0, current_position.1 + 2);
                grid[current_position.0][current_position.1] = '.';
                grid[current_position.0][current_position.1 - 1] = '|';
            },
            'S' => {
                current_position = (current_position.0 + 2, current_position.1);
                grid[current_position.0][current_position.1] = '.';
                grid[current_position.0 - 1][current_position.1] = '-';
            },
            'W' => {
                current_position = (current_position.0, current_position.1 - 2);
                grid[current_position.0][current_position.1] = '.';
                grid[current_position.0][current_position.1 + 1] = '|';
            },
            '(' => positions.push(current_position),
            ')' => current_position = positions.pop().unwrap(),
            '|' => current_position = positions[positions.len() - 1],
            _ => {}
        }
    }

    grid
}

fn eprint_grid(grid: &Grid) {
    let mut border_top = None;
    let mut border_right = None;
    let mut border_bottom = None;
    let mut border_left = None;

    for (x, row) in grid.iter().enumerate() {
        for (y, &tile) in row.iter().enumerate() {
            if tile == '-' || tile == '|' || tile == '.' || tile == 'X' {
                if border_top.is_none() || x < border_top.unwrap() {
                    border_top = Some(x);
                }

                if border_right.is_none() || y > border_right.unwrap() {
                    border_right = Some(y);
                }

                if border_bottom.is_none() || x > border_bottom.unwrap() {
                    border_bottom = Some(x);
                }

                if border_left.is_none() || y < border_left.unwrap() {
                    border_left = Some(y);
                }
            }
        }
    }

    if border_top.is_none() {
        return;
    }

    let border_top = border_top.unwrap();
    let border_right = border_right.unwrap();
    let border_bottom = border_bottom.unwrap();
    let border_left = border_left.unwrap();

    for row in grid.iter().skip(border_top - 1).take(border_bottom - border_top + 3) {
        for tile in row.iter().skip(border_left - 1).take(border_right - border_left + 3) {
            eprint!("{}", tile);
        }

        eprintln!();
    }

    eprintln!();
}

fn get_maximum_distance(grid: &Grid) -> usize {
    let mut seem: HashMap<(usize, usize), usize> = HashMap::new();
    let mut pending: HashMap<(usize, usize), usize> = HashMap::new();
    pending.insert(STARTING_POSITION, 0);
    let mut greater_distance = 0;

    loop {
        if pending.iter().len() == 0 {
            break;
        }

        let current_position = pending.keys().next().unwrap().clone();
        let distance = pending.remove(&current_position).unwrap().clone();

        add_pending_if_not_already_seem(&grid, &seem, &mut pending, current_position.0 - 2, current_position.1, current_position.0 - 1, current_position.1, distance + 1);
        add_pending_if_not_already_seem(&grid, &seem, &mut pending, current_position.0, current_position.1 + 2, current_position.0, current_position.1 + 1, distance + 1);
        add_pending_if_not_already_seem(&grid, &seem, &mut pending, current_position.0 + 2, current_position.1, current_position.0 + 1, current_position.1, distance + 1);
        add_pending_if_not_already_seem(&grid, &seem, &mut pending, current_position.0, current_position.1 - 2, current_position.0, current_position.1 - 1, distance + 1);

        if distance > greater_distance {
            greater_distance = distance;
        }

        seem.insert(current_position, distance);
    }

    greater_distance
}

fn add_pending_if_not_already_seem(grid: &Grid,
                                   seem: &HashMap<(usize, usize), usize>,
                                   pending: &mut HashMap<(usize, usize), usize>,
                                   x: usize,
                                   y: usize,
                                   divisor_x: usize,
                                   divisor_y: usize,
                                   distance: usize) {
    if grid[divisor_x][divisor_y] == '#' {
        return;
    }

    if seem.contains_key(&(x, y)) {
        return;
    }

    if grid[x][y] == '.' {
        pending.insert((x, y), distance);
    }
}

#[cfg(test)]
mod test;
