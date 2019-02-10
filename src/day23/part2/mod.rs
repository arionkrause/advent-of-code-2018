pub fn solve(input: &str) -> i64 {
    // Copied from https://github.com/mfs/aoc-2018/blob/master/src/bin/p23.rs
    let re = regex::Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();

    let mut bots: Vec<(i64, i64, i64, i64)> = vec![];

    for line in input.lines() {
        let c = re.captures(&line).unwrap();
        let nanobot = (c[1].parse().unwrap(), c[2].parse().unwrap(), c[3].parse().unwrap(), c[4].parse().unwrap());
        bots.push(nanobot);
    }

    bots.sort_unstable_by(|a, b| b.3.cmp(&a.3));

    // https://www.reddit.com/r/adventofcode/comments/a8s17l/2018_day_23_solutions/ecddus1/

    macro_rules! min_max {
        ($x:expr) => {
            (
                bots.iter().map($x).min().unwrap(),
                bots.iter().map($x).max().unwrap(),
            )
        };
    }

    let mut xs = min_max!(|a| a.0);
    let mut ys = min_max!(|a| a.1);
    let mut zs = min_max!(|a| a.2);

    let mut range = 1;
    while range < xs.1 - xs.0 {
        range *= 2;
    }

    loop {

        let mut target_count = 0;
        let mut best = (0, 0, 0);
        let mut best_val = 0;

        for x in (xs.0..=xs.1).step_by(range as usize) {
            for y in (ys.0..=ys.1).step_by(range  as usize) {
                for z in (zs.0..=zs.1).step_by(range as usize) {
                    let count = bots
                            .iter()
                            .filter(|b| (dis(&(x, y, z, 0), b) - b.3) / range as i64 <= 0)
                            .count();
                    if count > target_count {
                        // square with higher count
                        target_count = count;
                        best_val = x.abs() + y.abs() + z.abs();
                        best = (x, y, z);
                    } else if count == target_count {
                        // tie breaks, pick closest to origin
                        if x.abs() + y.abs() + z.abs() < best_val {
                            best_val = x.abs() + y.abs() + z.abs();
                            best = (x, y, z);
                        }
                    }
                }
            }
        }

        if range == 1 {
            return best_val;
        }

        xs = (best.0 - range, best.0 + range);
        ys = (best.1 - range, best.1 + range);
        zs = (best.2 - range, best.2 + range);

        range /= 2;
    }
}

fn dis(a: &(i64, i64, i64, i64), b: &(i64, i64, i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()
}

#[cfg(test)]
mod test;
