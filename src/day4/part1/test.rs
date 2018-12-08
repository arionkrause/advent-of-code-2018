use crate::day4::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day4/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), 240);
}

#[test]
fn test2() {
    let input = read_file("day4/input_test_part1_test2").unwrap();
    assert_eq!(solve(&input), 240);
}
