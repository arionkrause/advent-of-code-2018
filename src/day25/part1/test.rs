use crate::day25::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day25/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), 2);
}

#[test]
fn test2() {
    let input = read_file("day25/input_test_part1_test2").unwrap();
    assert_eq!(solve(&input), 4);
}

#[test]
fn test3() {
    let input = read_file("day25/input_test_part1_test3").unwrap();
    assert_eq!(solve(&input), 3);
}

#[test]
fn test4() {
    let input = read_file("day25/input_test_part1_test4").unwrap();
    assert_eq!(solve(&input), 8);
}
