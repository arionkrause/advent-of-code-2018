#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

struct Tile {
    tile_type: TileType,
    geologic_index: u32,
    erosion_level: u32,
}

enum TileType {
    Narrow,
    Rocky,
    Wet,
}

pub fn solve(input: &str) -> u32 {
    let (cave_depth, target_position) = decode_input(&input);
    eprintln!("cave_depth = {:?}", cave_depth);
    eprintln!("target_position = {:?}", target_position);

    let mut grid: Vec<Vec<Tile>> = Vec::with_capacity(target_position.y as usize);

    for y in 0..=target_position.y {
        let mut row: Vec<Tile> = Vec::with_capacity(target_position.x as usize);

        for x in 0..=target_position.x {
            let geologic_index = {
                if y == 0 && x == 0 {
                    0
                } else if y == target_position.y && x == target_position.x {
                    0
                } else if y == 0 {
                    x * 16807
                } else if x == 0 {
                    y * 48271
                } else {
                    row[x as usize - 1].erosion_level * grid[y as usize - 1][x as usize].erosion_level
                }
            };

            let erosion_level = ((geologic_index + cave_depth as u32) % 20183) as u32;

            let tile_type = match erosion_level % 3 {
                0 => TileType::Rocky,
                1 => TileType::Wet,
                2 => TileType::Narrow,
                _ => panic!(),
            };

            row.push(Tile {tile_type, geologic_index, erosion_level });
        }

        grid.push(row);
    }

    print_grid_type(&grid, &target_position);
    print_grid_geologic_index(&grid);
    print_grid_erosion_level(&grid);
    get_risk_level(&grid)
}

fn decode_input(input: &str) -> (u32, Position) {
    let re = regex::Regex::new(r"depth: (\d+)\r?\ntarget: (\d+),(\d+)").unwrap();
    let captures = re.captures(input).unwrap();
    let depth = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let x = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let y = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
    (depth, Position { x, y })
}

fn print_grid_type(grid: &Vec<Vec<Tile>>, target_position: &Position) {
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if y == 0 && x == 0 {
                eprint!("M");
            } else if y == target_position.y as usize && x == target_position.x as usize {
                eprint!("T");
            } else {
                eprint!("{}", match tile.tile_type {
                    TileType::Narrow => '|',
                    TileType::Rocky => '.',
                    TileType::Wet => '=',
                });
            }
        }

        eprintln!();
    }

    eprintln!();
}

fn print_grid_geologic_index(grid: &Vec<Vec<Tile>>) {
    for row in grid.iter() {
        for tile in row.iter() {
            eprint!("{:10}", tile.geologic_index);
        }

        eprintln!();
    }

    eprintln!();
}

fn print_grid_erosion_level(grid: &Vec<Vec<Tile>>) {
    for row in grid.iter() {
        for tile in row.iter() {
            eprint!("{:6}", tile.erosion_level);
        }

        eprintln!();
    }

    eprintln!();
}

fn get_risk_level(grid: &Vec<Vec<Tile>>) -> u32 {
    grid.iter().map(|row| row.iter()
            .map(|tile| match tile.tile_type {
                TileType::Rocky => 0,
                TileType::Wet => 1,
                TileType::Narrow => 2,
            }).sum::<u32>()
    ).sum()
}

#[cfg(test)]
mod test;
