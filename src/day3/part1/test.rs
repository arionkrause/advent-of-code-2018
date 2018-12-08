use crate::day3::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day3/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), 4);
}
