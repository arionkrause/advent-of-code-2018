use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Cart {
    id: usize,
    position: Position,
    direction: Direction,
    next_turn_choice: TurnChoice,
}

#[derive(Clone, Debug, Eq)]
struct Position {
    y: usize,
    x: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        if self.y != other.y {
            return self.y.cmp(&other.y);
        }

        return self.x.cmp(&other.x);
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Position) -> bool {
        self.y == other.y
                && self.x == other.x
    }
}

#[derive(Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Debug)]
enum TurnChoice {
    LEFT,
    STRAIGHT,
    RIGHT,
}

pub fn solve(input: &str, print_progress: bool) -> (usize, usize) {
    let (track, mut carts) = get_data(&input);
    let mut tick = 1;
    let mut _collision_position = None;

    'tick: loop {
        sort_carts(&mut carts);

        if print_progress {
            eprintln!("Tick: {:?}", tick);
            print_track(&track, &carts, None);
            print_carts(&carts);
            eprintln!();
        }

        let mut carts_positions = HashMap::new();

        for cart in carts.iter() {
            carts_positions.insert(cart.id, (cart.position.y, cart.position.x));
        }

        for cart in carts.iter_mut() {
            let (delta_y, delta_x): (isize, isize) = match cart.direction {
                Direction::UP => (-1, 0),
                Direction::RIGHT => (0, 1),
                Direction::DOWN => (1, 0),
                Direction::LEFT => (0, -1),
            };

            cart.position.y = (cart.position.y as isize + delta_y) as usize;
            cart.position.x = (cart.position.x as isize + delta_x) as usize;

            for (&other_cart_id, other_cart_position) in carts_positions.iter() {
                if cart.id == other_cart_id {
                    continue;
                }

                if cart.position.y == other_cart_position.0
                        && cart.position.x == other_cart_position.1 {
                    _collision_position = Some(cart.position.clone());
                    break 'tick;
                }
            }

            carts_positions.insert(cart.id, (cart.position.y, cart.position.x));

            match track[cart.position.y][cart.position.x] {
                '|' | '-' => {}
                '/' => {
                    match cart.direction {
                        Direction::UP => cart.direction = Direction::RIGHT,
                        Direction::RIGHT => cart.direction = Direction::UP,
                        Direction::DOWN => cart.direction = Direction::LEFT,
                        Direction::LEFT => cart.direction = Direction::DOWN,
                    };
                },
                '\\' => {
                    match cart.direction {
                        Direction::UP => cart.direction = Direction::LEFT,
                        Direction::RIGHT => cart.direction = Direction::DOWN,
                        Direction::DOWN => cart.direction = Direction::RIGHT,
                        Direction::LEFT => cart.direction = Direction::UP,
                    };
                },
                '+' => {
                    match cart.next_turn_choice {
                        TurnChoice::LEFT => {
                            match cart.direction {
                                Direction::UP => cart.direction = Direction::LEFT,
                                Direction::RIGHT => cart.direction = Direction::UP,
                                Direction::DOWN => cart.direction = Direction::RIGHT,
                                Direction::LEFT => cart.direction = Direction::DOWN,
                            }

                            cart.next_turn_choice = TurnChoice::STRAIGHT
                        },
                        TurnChoice::STRAIGHT => {
                            cart.next_turn_choice = TurnChoice::RIGHT
                        },
                        TurnChoice::RIGHT => {
                            match cart.direction {
                                Direction::UP => cart.direction = Direction::RIGHT,
                                Direction::RIGHT => cart.direction = Direction::DOWN,
                                Direction::DOWN => cart.direction = Direction::LEFT,
                                Direction::LEFT => cart.direction = Direction::UP,
                            }

                            cart.next_turn_choice = TurnChoice::LEFT
                        },
                    }
                },
                _ => panic!(),
            }
        }

        tick += 1;
    }

    let collision_position = _collision_position.unwrap();

    if print_progress {
        print_track(&track, &carts, Some(collision_position.clone()));
        print_carts(&carts);
    }

    return (collision_position.x, collision_position.y)
}

fn get_data(input: &str) -> (Vec<Vec<char>>, Vec<Cart>) {
    let mut track: Vec<Vec<char>> = Vec::new();
    let mut carts = Vec::new();
    let cart_identifiers = String::from("^>v<");
    let mut cart_id = 1;

    for (y, line) in input.lines().enumerate() {
        let mut track_row: Vec<char> = Vec::new();

        for (x, path) in line.chars().enumerate() {
            if cart_identifiers.contains(path) {
                let (cart, track_piece) = decode_cart(cart_id, path, y, x);
                carts.push(cart);
                track_row.push(track_piece);
                cart_id += 1;
            } else {
                track_row.push(path);
            }
        }

        track.push(track_row);
    }

    (track, carts)
}

fn decode_cart(cart_id: usize, cart_identifier: char, y: usize, x: usize) -> (Cart, char) {
    let (direction, path) = match cart_identifier {
        '^' => (Direction::UP, '|'),
        '>' => (Direction::RIGHT, '-'),
        'v' => (Direction::DOWN, '|'),
        '<' => (Direction::LEFT, '-'),
        _ => panic!(),
    };

    let cart = Cart {
        id: cart_id,
        position: Position {
            y,
            x,
        },
        direction,
        next_turn_choice: TurnChoice::LEFT,
    };

    (cart, path)
}

fn print_track(track: &Vec<Vec<char>>, carts: &Vec<Cart>, collision_position: Option<Position>) {
    let mut collision_position_y = None;
    let mut collision_position_x = None;

    if collision_position.is_some() {
        let collision_position = collision_position.unwrap();
        collision_position_y = Some(collision_position.y);
        collision_position_x = Some(collision_position.x);
    }

    for (y, row) in track.iter().enumerate() {
        for (x, path) in row.iter().enumerate() {
            if collision_position_y.is_some()
                    && collision_position_x.is_some()
                    && collision_position_y.unwrap() == y
                    && collision_position_x.unwrap() == x {
                eprint!("X");
                continue;
            }

            let mut there_is_a_cart_in_here = false;
            let mut cart_symbol = None;

            for cart in carts {
                if cart.position.y == y && cart.position.x == x {
                    there_is_a_cart_in_here = true;

                    cart_symbol = match cart.direction {
                        Direction::UP => Some('^'),
                        Direction::RIGHT => Some('>'),
                        Direction::DOWN => Some('v'),
                        Direction::LEFT => Some('<'),
                    };

                    break;
                }
            }

            if there_is_a_cart_in_here {
                eprint!("{}", cart_symbol.unwrap());
            } else {
                eprint!("{}", path);
            }
        }

        eprintln!();
    }
}

fn print_carts(carts: &Vec<Cart>) {
    for (index, cart) in carts.iter().enumerate() {
        eprintln!("Cart #{:?} - id: {:?} y: {:?}, x: {:?}, direction: {:?}, next_turn_choice: {:?}",
                 index, cart.id, cart.position.y, cart.position.x, cart.direction, cart.next_turn_choice);
    }
}

fn sort_carts(carts: &mut Vec<Cart>) {
    carts.sort_by(|a, b| a.position.cmp(&b.position));
}

#[cfg(test)]
mod test;
