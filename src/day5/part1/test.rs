use crate::day5::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day5/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), 10);
}
