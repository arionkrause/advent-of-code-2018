use regex::Regex;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Location {
    id: usize,
    x: usize,
    y: usize,
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Border {
    top: usize,
    right: usize,
    bottom: usize,
    left: usize,
}

pub fn solve(input: &str, maximum_total_distance: usize) -> usize {
    let locations= get_locations(&input);
    let border = get_border(&locations);
    get_target_region_size(&locations, &border, maximum_total_distance)
}

fn get_locations(input: &str) -> Vec<Location> {
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();
    let mut locations: Vec<Location> = Vec::new();

    for (index, line) in input.lines().enumerate() {
        let captures = re.captures(line).unwrap();
        let x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        locations.push(Location { id: index, x, y });
    }

    locations
}

fn get_border(locations: &[Location]) -> Border {
    let mut top: Option<usize> = None;
    let mut right: Option<usize> = None;
    let mut bottom: Option<usize> = None;
    let mut left: Option<usize> = None;

    for location in locations {
        if top == None || location.y < top.unwrap() {
            top = Some(location.y);
        }

        if right == None || location.x > right.unwrap() {
            right = Some(location.x);
        }

        if bottom == None || location.y > bottom.unwrap() {
            bottom = Some(location.y);
        }

        if left == None || location.x < left.unwrap() {
            left = Some(location.x);
        }
    }

    Border {top: top.unwrap(), right: right.unwrap(), bottom: bottom.unwrap(), left: left.unwrap() }
}

fn get_target_region_size(locations: &[Location], border: &Border, maximum_total_distance: usize) -> usize {
    let mut target_region_size = 0;

    for row in border.top..=border.bottom {
        for column in border.left..=border.right {
            let total_distance_to_all_locations = get_total_distance_to_all_locations(&locations, row, column);

            if total_distance_to_all_locations < maximum_total_distance {
                target_region_size += 1;
            }
        }
    }

    target_region_size
}

fn get_total_distance_to_all_locations(locations: &[Location], row: usize, column: usize) -> usize {
    let mut total_distance_to_all_locations = 0;

    for location in locations {
        total_distance_to_all_locations += get_manhattan_distance(location, row, column);
    }

    total_distance_to_all_locations
}

fn get_manhattan_distance(location: &Location, row: usize, column: usize) -> usize {
    let x_distance = i64::abs(location.x as i64 - row as i64) as usize;
    let y_distance = i64::abs(location.y as i64 - column as i64) as usize;
    x_distance + y_distance
}

#[cfg(test)]
mod test;
