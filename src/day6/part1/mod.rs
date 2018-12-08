use std::collections::HashSet;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Location {
    x: usize,
    y: usize,
}

pub fn solve(input: &str) -> usize {
    let locations: HashSet<Location> = get_locations(&input);
    let (top_left, bottom_right, border_touching_locations) = get_border_stats(&locations);
    let mut locations_scores: HashMap<Location, usize> = HashMap::new();

    for x in top_left.x..=bottom_right.x {
        for y in top_left.y..=bottom_right.y {
            if let Some(closest_location) = get_closest_location(&locations, x, y) {
                if border_touching_locations.contains(&closest_location) {
                    continue;
                }

                locations_scores.entry(closest_location)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }

    for y in top_left.y..=bottom_right.y {
        if let Some(closest_location) = get_closest_location(&locations, top_left.x, y) {
            let matching_locations: Vec<_> = locations_scores
                .iter()
                .filter(|&(l, _)| l == &closest_location)
                .map(|(k, _)| k.clone())
                .collect();

            for matching_location in matching_locations {
                locations_scores.remove(&matching_location);
            }
        }
    }

    for x in top_left.x..=bottom_right.x {
        if let Some(closest_location) = get_closest_location(&locations, x, bottom_right.y) {
            let matching_locations: Vec<_> = locations_scores
                .iter()
                .filter(|&(l, _)| l == &closest_location)
                .map(|(k, _)| k.clone())
                .collect();

            for matching_location in matching_locations {
                locations_scores.remove(&matching_location);
            }
        }
    }

    for y in top_left.y..=bottom_right.y {
        if let Some(closest_location) = get_closest_location(&locations, bottom_right.x, y) {
            let matching_locations: Vec<_> = locations_scores
                .iter()
                .filter(|&(l, _)| l == &closest_location)
                .map(|(k, _)| k.clone())
                .collect();

            for matching_location in matching_locations {
                locations_scores.remove(&matching_location);
            }
        }
    }

    for x in top_left.x..=bottom_right.x {
        if let Some(closest_location) = get_closest_location(&locations, x, top_left.y) {
            let matching_locations: Vec<_> = locations_scores
                .iter()
                .filter(|&(l, _)| l == &closest_location)
                .map(|(k, _)| k.clone())
                .collect();

            for matching_location in matching_locations {
                locations_scores.remove(&matching_location);
            }
        }
    }

    *locations_scores.iter().max_by(|&x, &y| x.1.cmp(y.1)).unwrap().1
}

fn get_locations(input: &str) -> HashSet<Location> {
    let re = Regex::new(r"^(\d+), (\d+)$").unwrap();
    let mut locations = HashSet::new();

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let y = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        locations.insert(Location {x, y});
    }

    locations
}

fn get_border_stats(locations: &HashSet<Location>) -> (Location, Location, HashSet<Location>) {
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

    let mut border_locations = HashSet::new();

    for location in locations {
        if location.x == left.unwrap() || location.x == right.unwrap() || location.y == top.unwrap() || location.y == bottom.unwrap() {
            border_locations.insert(location.clone());
        }
    }

    (Location { x: left.unwrap(), y: top.unwrap() }, Location { x: right.unwrap(), y: bottom.unwrap() }, border_locations)
}

fn get_closest_location(locations: &HashSet<Location>, x: usize, y: usize) -> Option<Location> {
    let mut distances: HashMap<Location, usize> = HashMap::new();
//eprintln!("x = {:?}, y = {:?}", x, y);
    for location in locations {
        if location.x == x && location.y == y {
            return Some(location.clone());
        }
    }

    for location in locations {
        let manhattan_distance = ((location.x as isize - x as isize).abs() + (location.y as isize - y as isize).abs()) as usize;
        distances.insert(location.clone(), manhattan_distance);
    }

    let smallest_distance_tuple = distances.iter().min_by(|&x, &y| x.1.cmp(y.1)).unwrap();

    if distances.iter().filter(|d| d.1 == smallest_distance_tuple.1).count() > 1 {
        return None;
    }

    Some(smallest_distance_tuple.0.clone())
}

#[cfg(test)]
mod test;
