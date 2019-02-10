#[derive(Debug)]
struct Nanobot {
    x: isize,
    y: isize,
    z: isize,
    radius: usize,
}

pub fn solve(input: &str) -> usize {
    let nanobots = decode_input(&input);
    let nanobot_with_strongest_signal = nanobots.iter().max_by(|a, b| a.radius.cmp(&b.radius)).unwrap();
    get_nanobots_in_range(&nanobots, &nanobot_with_strongest_signal)
}

fn get_nanobots_in_range(nanobots: &Vec<Nanobot>, nanobot: &Nanobot) -> usize {
    nanobots.iter().filter(|&some_nanobot| {
        get_manhattan_distance(&nanobot, &some_nanobot) <= nanobot.radius
    }).count()
}

fn get_manhattan_distance(from: &Nanobot, to: &Nanobot) -> usize {
    ((from.x - to.x).abs() + (from.y - to.y).abs() + (from.z - to.z).abs()) as usize
}

fn decode_input(input: &str) -> Vec<Nanobot> {
    let re = regex::Regex::new(r"pos=<(-?\d+),(-?\d+),(-?\d+)>, r=(\d+)").unwrap();
    let mut nanobots = Vec::new();

    for line in input.lines(){
        let captures = re.captures(&line).unwrap();

        nanobots.push(Nanobot {
            x: captures.get(1).unwrap().as_str().parse::<isize>().unwrap(),
            y: captures.get(2).unwrap().as_str().parse::<isize>().unwrap(),
            z: captures.get(3).unwrap().as_str().parse::<isize>().unwrap(),
            radius: captures.get(4).unwrap().as_str().parse::<usize>().unwrap(),
        });
    }

    nanobots
}

#[cfg(test)]
mod test;
