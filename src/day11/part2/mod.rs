pub fn solve(input: &str) -> (usize, usize, usize) {
    let serial = input.replace("\r\n", "").parse::<usize>().unwrap();
    let mut sum = [[0i32; 301]; 301];
    let mut max_total_power = None;
    let mut max_total_power_x = None;
    let mut max_total_power_y = None;
    let mut max_total_power_size = None;

    for y in 1..=300 {
        for x in 1..=300 {
            sum[y][x] = get_power_level(serial, x, y) + sum[y - 1][x] + sum[y][x - 1] - sum[y - 1][x - 1];
        }
    }

    for size in 1..=300 {
        for y in size..=300 {
            for x in size..=300 {
                let total_power = sum[y][x] - sum[y - size][x] - sum[y][x - size] + sum[y - size][x - size];

                if max_total_power.is_none() || total_power > max_total_power.unwrap() {
                    max_total_power = Some(total_power);
                    max_total_power_x = Some(x);
                    max_total_power_y = Some(y);
                    max_total_power_size = Some(size);
                }
            }
        }
    }

    (max_total_power_x.unwrap() - max_total_power_size.unwrap() + 1, max_total_power_y.unwrap() - max_total_power_size.unwrap() + 1, max_total_power_size.unwrap())
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
