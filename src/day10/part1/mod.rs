use regex::Regex;

#[derive(Clone, Debug)]
struct Point {
    position_x: i32,
    position_y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

#[derive(Clone, Debug)]
struct Border {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

pub fn solve(input: &str) -> String {
    let re = Regex::new(r"^position=<(.+?), (.+?)> velocity=<(.+?), (.+?)>").unwrap();
    let mut points = vec![];

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let position_x = i32::from_str_radix(captures.get(1).unwrap().as_str().to_string().trim_left(), 10).unwrap();
        let position_y = i32::from_str_radix(captures.get(2).unwrap().as_str().to_string().trim_left(), 10).unwrap();
        let velocity_x = i32::from_str_radix(captures.get(3).unwrap().as_str().to_string().trim_left(), 10).unwrap();
        let velocity_y = i32::from_str_radix(captures.get(4).unwrap().as_str().to_string().trim_left(), 10).unwrap();

        points.push(Point {
            position_x,
            position_y,
            velocity_x,
            velocity_y,
        });
    }

    let mut last_border_size = None;
    let mut last_points = None;
    let mut last_border = None;

    loop {
        let border = get_border(&points);
        let border_size = border.bottom - border.top + border.right - border.left;

        if last_border_size.is_none() || border_size < last_border_size.unwrap() {
            last_border_size = Some(border_size);
            last_points = Some(points.clone());
            last_border = Some(border.clone());
        } else {
            return get_formatted_points(&last_points.unwrap(), &last_border.unwrap());
        }

        move_points(&mut points);
    }
}

fn get_border(points: &[Point]) -> Border {
    let mut top = None;
    let mut right = None;
    let mut bottom = None;
    let mut left = None;

    for point in points.iter() {
        if top.is_none() || point.position_y < top.unwrap() {
            top = Some(point.position_y);
        }

        if right.is_none() || point.position_x > right.unwrap() {
            right = Some(point.position_x);
        }

        if bottom.is_none() || point.position_y > bottom.unwrap() {
            bottom = Some(point.position_y);
        }

        if left.is_none() || point.position_x < left.unwrap() {
            left = Some(point.position_x);
        }
    }

    Border {
        top: top.unwrap(),
        right: right.unwrap(),
        bottom: bottom.unwrap(),
        left: left.unwrap()
    }
}

fn get_formatted_points(points: &[Point], border: &Border) -> String {
    let mut formatted_points = String::new();

    for y in border.top..=border.bottom {
        for x in border.left..=border.right {
            let mut print = false;

            for point in points.iter() {
                if point.position_x == x && point.position_y == y {
                    print = true;
                    break;
                }
            }

            if print {
                formatted_points.push_str("#");
            } else {
                formatted_points.push_str(".");
            }
        }

        formatted_points.push_str("\n");
    }

    formatted_points
}

fn move_points(points: &mut [Point]) {
    for point in points.iter_mut() {
        point.position_x += point.velocity_x;
        point.position_y += point.velocity_y;
    }
}

#[cfg(test)]
mod test;
