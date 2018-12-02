use std::time::Instant;

mod io;
mod day1;

fn main() {
    let before = Instant::now();
    println!("Advent of code 2018");

    day1(); // Changes in frequency

    let after = before.elapsed();
    println!("Duration: {} ms", after.as_secs() * 1000 + (after.subsec_nanos() / 1_000_000) as u64);
}

fn day1() {
    let input = io::read_file("day1/input").unwrap();
    println!("Day 1, part 1: {}", day1::part1::solve(&input));
    println!("Day 1, part 2: {}", day1::part2::solve(&input));
}
