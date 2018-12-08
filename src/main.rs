use std::time::Instant;

mod io;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let before = Instant::now();
    println!("Advent of code 2018");

    day1(); // Chronal Calibration
    day2(); // Inventory Management System
    day3(); // No Matter How You Slice It
    day4(); // Repose Record
    day5(); // Alchemical Reduction
    day6(); // Chronal Coordinates

    let after = before.elapsed();
    println!("Duration: {} ms", after.as_secs() * 1000 + (after.subsec_nanos() / 1_000_000) as u64);
}

fn day1() {
    let input = io::read_file("day1/input").unwrap();
    println!("Day 1, part 1: {}", day1::part1::solve(&input));
    println!("Day 1, part 2: {}", day1::part2::solve(&input));
}

fn day2() {
    let input = io::read_file("day2/input").unwrap();
    println!("Day 2, part 1: {}", day2::part1::solve(&input));
    println!("Day 2, part 2: {}", day2::part2::solve(&input));
}

fn day3() {
    let input = io::read_file("day3/input").unwrap();
    println!("Day 3, part 1: {}", day3::part1::solve(&input));
    println!("Day 3, part 2: {}", day3::part2::solve(&input));
}

fn day4() {
    let input = io::read_file("day4/input").unwrap();
    println!("Day 4, part 1: {}", day4::part1::solve(&input));
    println!("Day 4, part 2: {}", day4::part2::solve(&input));
}

fn day5() {
    let input = io::read_file("day5/input").unwrap();
    println!("Day 5, part 1: {}", day5::part1::solve(&input));
    println!("Day 5, part 2: {}", day5::part2::solve(&input));
}

fn day6() {
    let input = io::read_file("day6/input").unwrap();
    println!("Day 6, part 1: {}", day6::part1::solve(&input));
    println!("Day 6, part 2: {}", day6::part2::solve(&input));
}
