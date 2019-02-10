use crate::day13::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day13/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input, true), (0, 3));
}

#[test]
fn test2() {
    let input = read_file("day13/input_test_part1_test2").unwrap();
    assert_eq!(solve(&input, true), (0, 4));
}

#[test]
fn test3() {
    let input = read_file("day13/input_test_part1_test3").unwrap();
    assert_eq!(solve(&input, true), (7, 3));
}
