use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

struct Tile {
    tile_type: TileType,
    geologic_index: u32,
    erosion_level: u32,
}

#[derive(Debug)]
struct Step {
    equipment: Equipment,
    to: Position,
    equipment_at_arrival: Equipment,
    duration: u32,
}

enum TileType {
    Narrow,
    Rocky,
    Wet,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Equipment {
    ClimbingGear,
    Neither,
    Torch,
}

pub fn solve(input: &str) -> u32 {
    // calculated answer: 972
    // correct answer: 976
    panic!();
    let (cave_depth, target_position) = decode_input(&input);
    let mut grid: Vec<Vec<Tile>> = Vec::with_capacity(target_position.y as usize + 10);

    for y in 0..=target_position.y + 10 {
        let mut row: Vec<Tile> = Vec::with_capacity(target_position.x as usize + 10);

        for x in 0..=target_position.x + 10 {
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

            row.push(Tile { tile_type, geologic_index, erosion_level });
        }

        grid.push(row);
    }

    get_fewest_minutes_to_reach_target(&grid, &target_position)
}

fn decode_input(input: &str) -> (u32, Position) {
    let re = regex::Regex::new(r"depth: (\d+)\r?\ntarget: (\d+),(\d+)").unwrap();
    let captures = re.captures(input).unwrap();
    let depth = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let x = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let y = captures.get(3).unwrap().as_str().parse::<u32>().unwrap();
    (depth, Position { x, y })
}

fn get_fewest_minutes_to_reach_target(grid: &Vec<Vec<Tile>>, target_position: &Position) -> u32 {
    let start = (Position { x: 0, y: 0 }, Equipment::Torch);
    let mut open_set = HashSet::new();
    open_set.insert(start.clone());
    let mut closed_set = Vec::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    g_score.insert(start.clone(), 0);
    let mut f_score = HashMap::new();
    f_score.insert(start.clone(), get_manhattan_distance(&start.0, &target_position));
    let mut i = 1;

    loop {
        if i % 1000 == 0 {
            eprintln!("i = {:?}", i);
        }
//        eprintln!("open_set = {:?}", open_set);
//        eprintln!("closed_set = {:?}", closed_set);
//        eprintln!("g_score = {:?}", g_score);
//        eprintln!("f_score = {:?}", f_score);
//        eprintln!("came_from = {:?}", came_from);
//        eprintln!();

        if open_set.len() == 0 {
            panic!("Impossible to reach target position.");
        };

        let current_id = f_score.iter()
                .filter(|&score| open_set.contains(score.0))
                .min_by(|a, b| a.1.cmp(&b.1)).unwrap().0.clone();

        let current_position = &current_id.0;
        let current_equipment = &current_id.1;

        if current_position == target_position {
            print_path(&grid, &current_id, &target_position, &came_from, &start);
            return g_score.get(&current_id).unwrap().clone();
        }

        open_set.remove(&current_id);
        closed_set.push(current_id.clone());

        for step in get_steps(&grid, &current_position, &current_equipment, &target_position).iter() {
            let step_id = &(step.to.clone(), step.equipment_at_arrival.clone());

            if closed_set.contains(&step_id) {
                continue;
            }

            let tentative_g_score = g_score.get(&current_id).unwrap() + step.duration;

            if !open_set.contains(&step_id) {
                open_set.insert(step_id.clone());
            } else if tentative_g_score >= *g_score.get(&step_id).unwrap() {
                continue;
            }

            came_from.insert(step_id.clone(), current_id.clone());
            g_score.insert(step_id.clone(), tentative_g_score);
            f_score.insert(step_id.clone(), g_score.get(step_id).unwrap() + get_manhattan_distance(&step.to, target_position));
        }

        i += 1;
    }
}

fn print_path(grid: &Vec<Vec<Tile>>, current_id: &(Position, Equipment), target_position: &Position, came_from: &HashMap<(Position, Equipment), (Position, Equipment)>, start: &(Position, Equipment)) {
    let current_position = &current_id.0.clone();
    let path = get_path(came_from, &current_id, &start);

    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if x == 0 && y == 0 {
                eprint!("M");
            } else if x == target_position.x as usize && y == target_position.y as usize {
                eprint!("T");
            } else if x == current_position.x as usize && y == current_position.y as usize {
                eprint!("O");
            } else if path.contains(&Position { x: x as u32, y: y as u32 }) {
                eprint!("*");
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

fn get_path(came_from: &HashMap<(Position, Equipment), (Position, Equipment)>, current_id: &(Position, Equipment), start: &(Position, Equipment)) -> Vec<Position> {
    let mut path = Vec::new();
    let previous = came_from.get(&current_id);

    if previous.is_none() || previous.unwrap() == start {
        path.push(start.0.clone());
    } else {
        path.extend(get_path(&came_from, &previous.unwrap(), &start));
    }

    path.push(current_id.0.clone());
    path
}

fn get_manhattan_distance(from: &Position, to: &Position) -> u32 {
    return ((to.x as i32 - from.x as i32).abs() + (to.y as i32 - from.y as i32).abs()) as u32;
}

fn get_steps(grid: &Vec<Vec<Tile>>, from_position: &Position, current_equipment: &Equipment, target_position: &Position) -> Vec<Step> {
    let mut steps = Vec::new();

    if from_position.y > 0 {
        steps.extend(get_possible_steps(&grid, Position { x: from_position.x, y: from_position.y - 1 }, &current_equipment, &target_position));
    }

    if (from_position.x as usize) < grid[0].len() - 1 {
        steps.extend(get_possible_steps(&grid, Position { x: from_position.x + 1, y: from_position.y }, &current_equipment, &target_position));
    }

    if (from_position.y as usize) < grid.len() - 1 {
        steps.extend(get_possible_steps(&grid, Position { x: from_position.x, y: from_position.y + 1 }, &current_equipment, &target_position));
    }

    if from_position.x > 0 {
        steps.extend(get_possible_steps(&grid, Position { x: from_position.x - 1, y: from_position.y }, &current_equipment, &target_position));
    }

    steps
}

fn get_possible_steps(grid: &Vec<Vec<Tile>>, to_position: Position, equipment: &Equipment, target_position: &Position) -> Vec<Step> {
    let mut possible_steps = Vec::new();

    match grid[to_position.y as usize][to_position.x as usize].tile_type {
        TileType::Narrow => {
            match equipment {
                Equipment::ClimbingGear => {
                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Neither,
                        duration: 8,
                    });

                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Torch,
                        duration: 8,
                    });
                }
                Equipment::Neither => {
                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Neither,
                        duration: 1,
                    });

                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Torch,
                        duration: 8,
                    });
                }
                Equipment::Torch => {
                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Neither,
                        duration: 8,
                    });

                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Torch,
                        duration: 1,
                    });
                }
            }
        }
        TileType::Rocky => {
            if &to_position == target_position {
                match equipment {
                    Equipment::ClimbingGear => {
                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::Torch,
                            duration: 8,
                        });
                    }
                    Equipment::Neither => {
                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::Torch,
                            duration: 8,
                        });
                    }
                    Equipment::Torch => {
                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::Torch,
                            duration: 1,
                        });
                    }
                }
            } else {
                match equipment {
                    Equipment::ClimbingGear => {
                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::ClimbingGear,
                            duration: 1,
                        });

                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::Torch,
                            duration: 8,
                        });
                    }
                    Equipment::Neither => {
                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::ClimbingGear,
                            duration: 8,
                        });

                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::Torch,
                            duration: 8,
                        });
                    }
                    Equipment::Torch => {
                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::ClimbingGear,
                            duration: 8,
                        });

                        possible_steps.push(Step {
                            equipment: equipment.clone(),
                            to: to_position.clone(),
                            equipment_at_arrival: Equipment::Torch,
                            duration: 1,
                        });
                    }
                }
            }
        }
        TileType::Wet => {
            match equipment {
                Equipment::ClimbingGear => {
                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::ClimbingGear,
                        duration: 1,
                    });

                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Neither,
                        duration: 8,
                    });
                }
                Equipment::Neither => {
                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::ClimbingGear,
                        duration: 8,
                    });

                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Neither,
                        duration: 1,
                    });
                }
                Equipment::Torch => {
                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::ClimbingGear,
                        duration: 8,
                    });

                    possible_steps.push(Step {
                        equipment: equipment.clone(),
                        to: to_position.clone(),
                        equipment_at_arrival: Equipment::Neither,
                        duration: 8,
                    });
                }
            }
        }
    }

    possible_steps
}

#[cfg(test)]
mod test;
