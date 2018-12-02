use crate::day1::part2::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day1/input_test_part2_test1").unwrap();
    assert_eq!(solve(&input), 2);
}

#[test]
fn test2() {
    let input = read_file("day1/input_test_part2_test2").unwrap();
    assert_eq!(solve(&input), 0);
}

#[test]
fn test3() {
    let input = read_file("day1/input_test_part2_test3").unwrap();
    assert_eq!(solve(&input), 10);
}

#[test]
fn test4() {
    let input = read_file("day1/input_test_part2_test4").unwrap();
    assert_eq!(solve(&input), 5);
}

#[test]
fn test5() {
    let input = read_file("day1/input_test_part2_test5").unwrap();
    assert_eq!(solve(&input), 14);
}
