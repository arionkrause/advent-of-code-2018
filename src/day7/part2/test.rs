use crate::day7::part2::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day7/input_test_part2_test1").unwrap();
    assert_eq!(solve(&input, 2, 60), 15);
}
