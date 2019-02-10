use crate::day11::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day11/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), (237, 0, 31)); // Calculated a posteriori
}

#[test]
fn test2() {
    let input = read_file("day11/input_test_part1_test2").unwrap();
    assert_eq!(solve(&input), (33, 45, 29));
}

#[test]
fn test3() {
    let input = read_file("day11/input_test_part1_test3").unwrap();
    assert_eq!(solve(&input), (21, 61, 30));
}
