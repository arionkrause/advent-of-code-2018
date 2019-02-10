use crate::day11::part2::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day11/input_test_part2_test1").unwrap();
    assert_eq!(solve(&input), (90, 269, 16));
}

#[test]
fn test2() {
    let input = read_file("day11/input_test_part2_test2").unwrap();
    assert_eq!(solve(&input), (232, 251, 12));
}
