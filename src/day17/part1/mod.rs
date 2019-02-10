use regex::Regex;

#[derive(Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

pub fn solve(input: &str) -> usize {
    // Credits: galenelias
    let mut map = decode_map(&input);
    while drip(&mut map, 1, 500) {}
    print_map(&map, None);
    get_amount_tiles_reached_by_water(&map)
    // manually dismiss water tiles above topmost clay
}

fn drip(map: &mut Vec<Vec<char>>, y_start: usize, x_start: usize) -> bool {
    for y in y_start..map.len() {
        if map[y][x_start] == '|' || map[y][x_start] == '.' {
            map[y][x_start] = '|';
            continue;
        }

        // Hit something, start flowing. Check if we hit walls, then fill with '~', else, spread '|' and recurse
        let flow_y = y - 1;
        let mut made_progress = false;
        let mut hit_left_wall = false;
        let mut hit_right_wall = false;

        for x in (0..x_start).rev() {
            if map[flow_y][x] == '#' {
                hit_left_wall = true;
                break;
            }

            map[flow_y][x] = '|';

            if map[y][x] != '#' && map[y][x] != '~' {
                if drip(map, flow_y, x) {
                    made_progress = true;
                }

                break;
            }
        }

        for x in x_start..map[0].len() {
            if map[flow_y][x] == '#' {
                hit_right_wall = true;
                break;
            }

            map[flow_y][x] = '|';

            if map[y][x] != '#' && map[y][x] != '~' {
                if drip(map, flow_y, x) {
                    made_progress = true;
                }

                break;
            }
        }

        // If we hit walls, then switch our '|'s to '~'s
        if hit_left_wall && hit_right_wall {
            made_progress = true;

            for x in (0..x_start).rev() {
                if map[flow_y][x] != '|' {
                    break;
                }

                map[flow_y][x] = '~';
            }

            for x in x_start..map[flow_y].len() {
                if map[flow_y][x] != '|' {
                    break;
                }

                map[flow_y][x] = '~';
            }
        }

        return made_progress;
    }

    return false;
}

fn decode_map(input: &str) -> Vec<Vec<char>> {
    let re_x = Regex::new(r"^x=(\d+), y=(\d+)\.\.(\d+)$").unwrap();
    let re_y = Regex::new(r"^y=(\d+), x=(\d+)\.\.(\d+)$").unwrap();
    let mut clay_points = Vec::new();

    for line in input.lines() {
        if let Some(captures) = re_x.captures(line) {
            let x = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let y_start = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let y_end = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

            for y in y_start..=y_end {
                clay_points.push((y, x));
            }
        } else if let Some(captures) = re_y.captures(line) {
            let y = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let x_start = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let x_end = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

            for x in x_start..=x_end {
                clay_points.push((y, x));
            }
        } else {
            panic!()
        }
    }

    let bottom = clay_points.iter()
            .max_by(|&a, &b| a.0.cmp(&b.0))
            .unwrap()
            .0 + 1;

    let right = clay_points.iter()
            .max_by(|&a, &b| a.1.cmp(&b.1))
            .unwrap()
            .1 + 2;

    let mut map = vec![vec!['.'; right]; bottom];

    for (y, x) in clay_points {
        map[y][x] = '#';
    }

    map[0][500] = '+';
    map
}

fn print_map(map: &Vec<Vec<char>>, point_of_interest: Option<Position>) {
    let mut leftmost_clay_x = None;

    let mut water_y = None;
    let mut water_x = None;

    if point_of_interest.is_some() {
        let water = point_of_interest.unwrap();
        water_y = Some(water.y);
        water_x = Some(water.x);
    }

    for row in map.iter() {
        for (index, &column) in row.iter().enumerate() {
            if column == '#' && (leftmost_clay_x.is_none() || index < leftmost_clay_x.unwrap()) {
                leftmost_clay_x = Some(index);
            }
        }
    }

    let leftmost_clay_x = leftmost_clay_x.unwrap();

    for (y, row) in map.iter().enumerate() {
        for (x, column) in row.iter().enumerate().skip(leftmost_clay_x) {
            if water_y.is_some() && water_y.unwrap() == y
                    && water_x.is_some() && water_x.unwrap() == x {
                print!("o");
            } else {
                print!("{}", column);
            }
        }

        println!();
    }

    println!();
}

fn get_amount_tiles_reached_by_water(map: &Vec<Vec<char>>) -> usize {
    let mut total = 0;

    for row in map.iter() {
        total += row.iter()
                .filter(|&&column| column == '~' || column == '|')
                .count();
    }

    total
}

#[cfg(test)]
mod test;
