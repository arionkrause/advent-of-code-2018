use crate::day16::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day16/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), 1);
}
