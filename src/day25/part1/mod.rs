#[derive(Clone, Debug)]
struct Point {
    x: i8,
    y: i8,
    z: i8,
    w: i8,
}

impl Point {
    fn manhattan_distance_to(&self, point: &Point) -> i8 {
        (self.x - point.x).abs()
        + (self.y - point.y).abs()
        + (self.z - point.z).abs()
        + (self.w - point.w).abs()
    }
}

pub fn solve(input: &str) -> usize {
    let points = decode_input(&input);
    let constellations = get_constellations(&points);
    constellations.len()
}

fn get_constellations(points: &Vec<Point>) -> Vec<Vec<Point>> {
    let mut constellations: Vec<Vec<Point>> = Vec::new();

    for point in points.iter() {
        let mut containing_constellations = Vec::new();

        for (constellation_index, constellation) in constellations.iter().enumerate() {
            for other_point in constellation.iter() {
                if point.manhattan_distance_to(other_point) <= 3 {
                    containing_constellations.push(constellation_index);
                    break;
                }
            }
        }

        if containing_constellations.is_empty() {
            constellations.push(vec![point.clone()]);
        } else {
            constellations[containing_constellations[0]].push(point.clone());

            for &constellation_index in containing_constellations.iter().skip(1).rev() {
                let merging_constellation = constellations.remove(constellation_index);
                constellations[containing_constellations[0]].extend(merging_constellation);
            }
        }
    }

    constellations
}

fn decode_input(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
        let coordinates: Vec<i8> = line.split(',').map(|c| c.parse().unwrap()).collect();

        Point {
            x: coordinates[0],
            y: coordinates[1],
            z: coordinates[2],
            w: coordinates[3],
        }
    }).collect()
}


//use std::collections::HashSet;
//
//pub fn solve(input: &str) -> usize {
//    let mut points: Vec<(i8, i8, i8, i8, i8)> = input.lines().enumerate().map(|(i, line)| {
//        let el: Vec<i8> = line.split(',').map(|c| c.parse().unwrap()).collect();
//        (el[0], el[1], el[2], el[3], i as i8)
//    }).collect();
//
//    for i in 0..points.len() {
//        for j in i + 1..points.len() {
//            if points[i].4 != points[j].4 && (points[i].0 - points[j].0).abs() + (points[i].1 - points[j].1).abs() + (points[i].2 - points[j].2).abs() + (points[i].3 - points[j].3).abs() <= 3 {
//                for k in 0..points.len() {
//                    points[k].4 = if points[k].4 == points[j].4 { points[i].4 } else { points[k].4 };
//                }
//            }
//        }
//    }
//
//    points.iter().map(|x| x.4).collect::<HashSet<i8>>().len()
//}

#[cfg(test)]
mod test;
