use regex::Regex;
use std::collections::HashMap;
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

pub fn solve(input: &str) -> usize {
    let locations= get_locations(&input);
    let border = get_border(&locations);
    let locations_with_infinite_areas = get_locations_with_infinite_areas(&locations, &border);
    let finite_areas = get_finite_areas(&locations, &border, &locations_with_infinite_areas);
    *finite_areas.iter().max_by(|&a, &b| a.1.cmp(b.1)).unwrap().1
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

fn get_locations_with_infinite_areas<'a>(locations: &'a[Location], border: &Border) -> Vec<&'a Location>{
    let mut locations_with_infinite_areas = Vec::new();
    let mut current_x = border.left;
    let mut current_y = border.top;

    while current_x < border.right {
        match get_closest_location(locations, current_x, current_y) {
            Some(closest_location) => locations_with_infinite_areas.push(closest_location),
            _ => {},
        }

        current_x += 1;
    }

    while current_y < border.bottom {
        match get_closest_location(locations, current_x, current_y) {
            Some(closest_location) => locations_with_infinite_areas.push(closest_location),
            _ => {},
        }

        current_y += 1;
    }

    while current_x > border.left {
        match get_closest_location(locations, current_x, current_y) {
            Some(closest_location) => locations_with_infinite_areas.push(closest_location),
            _ => {},
        }

        current_x -= 1;
    }

    while current_y > border.top {
        match get_closest_location(locations, current_x, current_y) {
            Some(closest_location) => locations_with_infinite_areas.push(closest_location),
            _ => {},
        }

        current_y -= 1;
    }

    locations_with_infinite_areas.sort();
    locations_with_infinite_areas.dedup();
    locations_with_infinite_areas

}

fn get_closest_location(locations: &[Location], row: usize, column: usize) -> Option<&Location> {
    let mut manhattan_distances = HashMap::new();

    for location in locations {
        manhattan_distances.insert(location, get_manhattan_distance(location, row, column));
    }

    let mut closest_manhattan_distance_location: Option<(&Location, usize)> = None;

    for (location, manhattan_distance) in manhattan_distances.iter() {
        if closest_manhattan_distance_location == None || manhattan_distance < &closest_manhattan_distance_location.unwrap().1 {
            closest_manhattan_distance_location = Some((*location, *manhattan_distance));
        }
    }

    let mut amount_locations_closest_manhattan_distance = 0;

    for (_, manhattan_distance) in manhattan_distances.iter() {
        if manhattan_distance == &closest_manhattan_distance_location.unwrap().1 {
            amount_locations_closest_manhattan_distance += 1;
        }
    }

    if amount_locations_closest_manhattan_distance > 1 {
        return None;
    }

    Some(closest_manhattan_distance_location.unwrap().0)
}

fn get_manhattan_distance(location: &Location, row: usize, column: usize) -> usize {
    let x_distance = i64::abs(location.x as i64 - row as i64) as usize;
    let y_distance = i64::abs(location.y as i64 - column as i64) as usize;
    x_distance + y_distance
}

fn get_finite_areas<'a>(locations: &'a [Location], border: &Border, locations_with_infinite_areas: &Vec<&'a Location>) -> HashMap<&'a Location, usize> {
    let mut finite_areas = HashMap::new();

    for row in border.top..=border.bottom {
        for column in border.left..=border.right {
            match get_closest_location(locations, row, column) {
                Some(closest_location) => {
                    if !locations_with_infinite_areas.contains(&closest_location) {
                        finite_areas.entry(closest_location)
                            .and_modify(|e| { *e += 1 })
                            .or_insert(1);
                    }
                },
                _ => {},
            }
        }
    }

    finite_areas
}

#[cfg(test)]
mod test;
