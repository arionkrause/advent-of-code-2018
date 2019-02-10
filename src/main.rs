use std::time::Instant;

mod io;
//mod day1;
//mod day2;
//mod day3;
//mod day4;
//mod day5;
//mod day6;
//mod day7;
//mod day8;
//mod day9;
//mod day10;
//mod day11;
//mod day12;
//mod day13;
//mod day14;
//mod day15;
//mod day16;
//mod day17;
//mod day18;
//mod day19;
//mod day20;
//mod day21;
//mod day22;
//mod day23;
//mod day24;
mod day25;

fn main() {
    let before = Instant::now();
    println!("Advent of code 2018");

//    day1(); // Chronal Calibration
//    day2(); // Inventory Management System
//    day3(); // No Matter How You Slice It
//    day4(); // Repose Record
//    day5(); // Alchemical Reduction
//    day6(); // Chronal Coordinates
//    day7(); // The Sum of Its Parts
//    day8(); // Memory Maneuver
//    day9(); // Marble Mania
//    day10(); // The Stars Align
//    day11(); // Chronal Charge
//    day12(); // Subterranean Sustainability
//    day13(); // Mine Cart Madness
//    day14(); // Chocolate Charts
//    day15(); // Beverage Bandits
//    day16(); // Chronal Classification
//    day17(); // Reservoir Research
//    day18(); // Settlers of The North Pole
//    day19(); // Go With The Flow
//    day20(); // A Regular Map
//    day21(); // Chronal Conversion
//    day22(); // Mode Maze
//    day23(); // Experimental Emergency Teleportation
//    day24(); // Immune System Simulator 20XX
    day25(); // Four-Dimensional Adventure

    let after = before.elapsed();
    println!("Duration: {} ms", after.as_secs() * 1000 + (after.subsec_nanos() / 1_000_000) as u64);
}

//fn day1() {
//    let input = io::read_file("day1/input").unwrap();
//    println!("Day 1, part 1: {}", day1::part1::solve(&input));
//    println!("Day 1, part 2: {}", day1::part2::solve(&input));
//}
//
//fn day2() {
//    let input = io::read_file("day2/input").unwrap();
//    println!("Day 2, part 1: {}", day2::part1::solve(&input));
//    println!("Day 2, part 2: {}", day2::part2::solve(&input));
//}
//
//fn day3() {
//    let input = io::read_file("day3/input").unwrap();
//    println!("Day 3, part 1: {}", day3::part1::solve(&input));
//    println!("Day 3, part 2: {}", day3::part2::solve(&input));
//}
//
//fn day4() {
//    let input = io::read_file("day4/input").unwrap();
//    println!("Day 4, part 1: {}", day4::part1::solve(&input));
//    println!("Day 4, part 2: {}", day4::part2::solve(&input));
//}
//
//fn day5() {
//    let input = io::read_file("day5/input").unwrap();
//    println!("Day 5, part 1: {}", day5::part1::solve(&input));
//    println!("Day 5, part 2: {}", day5::part2::solve(&input));
//}
//
//fn day6() {
//    let input = io::read_file("day6/input").unwrap();
//    println!("Day 6, part 1: {}", day6::part1::solve(&input));
//    println!("Day 6, part 2: {}", day6::part2::solve(&input, 10000));
//}
//
//fn day7() {
//    let input = io::read_file("day7/input").unwrap();
//    println!("Day 7, part 1: {}", day7::part1::solve(&input));
//    println!("Day 7, part 2: {}", day7::part2::solve(&input, 5, 0));
//}
//
//fn day8() {
//    let input = io::read_file("day8/input").unwrap();
//    println!("Day 8, part 1: {}", day8::part1::solve(&input));
//    println!("Day 8, part 2: {}", day8::part2::solve(&input));
//}
//
//fn day9() {
//    let input = io::read_file("day9/input").unwrap();
//    println!("Day 9, part 1: {}", day9::part1::solve(&input));
//    println!("Day 9, part 2: {}", day9::part2::solve(&input));
//}
//
//fn day10() {
//    let input = io::read_file("day10/input").unwrap();
//    println!("Day 10, part 1: \n{}", day10::part1::solve(&input));
//    println!("Day 10, part 2: {}", day10::part2::solve(&input));
//}
//
//fn day11() {
//    let input = io::read_file("day11/input").unwrap();
//    println!("Day 11, part 1: {:?}", day11::part1::solve(&input));
//    println!("Day 11, part 2: {:?}", day11::part2::solve(&input));
//}
//
//fn day12() {
//    let input = io::read_file("day12/input").unwrap();
//    println!("Day 12, part 1: {:?}", day12::part1::solve(&input, 20));
//    println!("Day 12, part 2: {:?}", day12::part2::solve(&input, 50_000_000_000));
//}
//
//fn day13() {
//    let input = io::read_file("day13/input").unwrap();
//    println!("Day 13, part 1: {:?}", day13::part1::solve(&input, false));
//    println!("Day 13, part 2: {:?}", day13::part2::solve(&input, false));
//}
//
//fn day14() {
//    let input = io::read_file("day14/input").unwrap();
//    println!("Day 14, part 1: {:?}", day14::part1::solve(&input));
//    println!("Day 14, part 2: {:?}", day14::part2::solve(&input));
//}
//
//fn day15() {
//    let input = io::read_file("day15/input").unwrap();
//    println!("Day 15, part 1: {:?}", day15::part1::solve(&input));
//    println!("Day 15, part 2: {:?}", day15::part2::solve(&input));
//}
//
//fn day16() {
//    let input = io::read_file("day16/input").unwrap();
//    println!("Day 16, part 1: {:?}", day16::part1::solve(&input));
//    println!("Day 16, part 2: {:?}", day16::part2::solve(&input));
//}
//
//fn day17() {
//    let input = io::read_file("day17/input").unwrap();
//    println!("Day 17, part 1: {:?}", day17::part1::solve(&input));
//    println!("Day 17, part 2: {:?}", day17::part2::solve(&input));
//}
//
//fn day18() {
//    let input = io::read_file("day18/input").unwrap();
//    println!("Day 18, part 1: {:?}", day18::part1::solve(&input, 10));
//    println!("Day 18, part 2: {:?}", day18::part2::solve(&input, 1_000_000_000));
//}
//
//fn day19() {
//    let input = io::read_file("day19/input").unwrap();
//    println!("Day 19, part 1: {:?}", day19::part1::solve(&input));
//    println!("Day 19, part 2: {:?}", day19::part2::solve(&input));
//}
//
//fn day20() {
//    let input = io::read_file("day20/input").unwrap();
////    println!("Day 20, part 1: {:?}", day20::part1::solve(&input));
//    println!("Day 20, part 2: {:?}", day20::part2::solve(&input));
//}
//
//fn day21() {
//    let input = io::read_file("day21/input").unwrap();
//    println!("Day 21, part 1: {:?}", day21::part1::solve(&input));
//    println!("Day 21, part 2: {:?}", day21::part2::solve(&input));
//}
//
//fn day22() {
//    let input = io::read_file("day22/input").unwrap();
//    println!("Day 22, part 1: {:?}", day22::part1::solve(&input));
//    println!("Day 22, part 2: {:?}", day22::part2::solve(&input));
//}
//
//fn day23() {
//    let input = io::read_file("day23/input").unwrap();
//    println!("Day 23, part 1: {:?}", day23::part1::solve(&input));
//    println!("Day 23, part 2: {:?}", day23::part2::solve(&input));
//}
//
//fn day24() {
//    let input = io::read_file("day24/input").unwrap();
//    println!("Day 24, part 1: {:?}", day24::part1::solve(&input));
//    println!("Day 24, part 2: {:?}", day24::part2::solve(&input));
//}
//
fn day25() {
    let input = io::read_file("day25/input").unwrap();
    println!("Day 25, part 1: {:?}", day25::part1::solve(&input));
//    println!("Day 25, part 2: {:?}", day25::part2::solve(&input));
}
