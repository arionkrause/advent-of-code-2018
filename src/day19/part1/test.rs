use crate::day19::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day19/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), 6);
}
