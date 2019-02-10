use crate::day22::part2::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day22/input_test_part2_test1").unwrap();
    assert_eq!(solve(&input), 45);
}
