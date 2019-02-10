use crate::day8::part2::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day8/input_test_part2_test1").unwrap();
    assert_eq!(solve(&input), 66);
}
