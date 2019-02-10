pub fn solve(input: &str) -> (usize, usize, i32) {
    let serial = input.replace("\r\n", "").parse::<usize>().unwrap();
    let mut grid = Vec::with_capacity(300 * 300);
    let mut max_total_power = None;
    let mut max_total_power_x = None;
    let mut max_total_power_y = None;

    for y in 0..300 {
        for x in 0..300 {
            grid.push(get_power_level(serial, x, y));
        }
    }

    for y in 0..298 {
        for x in 0..298 {
            let total_power = get_total_power(&grid, x, y);

            if max_total_power.is_none() || total_power > max_total_power.unwrap() {
                max_total_power = Some(total_power);
                max_total_power_x = Some(x);
                max_total_power_y = Some(y);
            }
        }
    }

    (max_total_power_x.unwrap(), max_total_power_y.unwrap(), max_total_power.unwrap())
}

fn get_total_power(grid: &[i32], top_left_x: usize, top_left_y: usize) -> i32 {
    let mut total_power = 0;

    for y in top_left_y..top_left_y + 3 {
        total_power += grid[y * 300 + top_left_x..y * 300 + top_left_x + 3].iter().sum::<i32>();
    }

    total_power
}

fn get_power_level(serial: usize, x: usize, y: usize) -> i32 {
    let rack_id = x as i32 + 10;
    let mut power_level: i32 = rack_id * y as i32;
    power_level += serial as i32;
    power_level *= rack_id;

    power_level = match power_level.to_string().chars().rev().nth(2) {
        Some(v) => v.to_digit(10).unwrap() as i32,
        None => 0,
    };

    power_level -= 5;
    power_level
}

#[cfg(test)]
mod test;
