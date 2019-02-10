use crate::day9::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day9/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), 32);
}

#[test]
fn test2() {
    let input = read_file("day9/input_test_part1_test2").unwrap();
    assert_eq!(solve(&input), 8317);
}

#[test]
fn test3() {
    let input = read_file("day9/input_test_part1_test3").unwrap();
    assert_eq!(solve(&input), 146373);
}

#[test]
fn test4() {
    let input = read_file("day9/input_test_part1_test4").unwrap();
    assert_eq!(solve(&input), 2764);
}

#[test]
fn test5() {
    let input = read_file("day9/input_test_part1_test5").unwrap();
    assert_eq!(solve(&input), 54718);
}

#[test]
fn test6() {
    let input = read_file("day9/input_test_part1_test6").unwrap();
    assert_eq!(solve(&input), 37305);
}
