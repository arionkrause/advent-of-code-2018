use crate::day14::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day14/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), 5158916779);
}

#[test]
fn test2() {
    let input = read_file("day14/input_test_part1_test2").unwrap();
    assert_eq!(solve(&input), 0124515891);
}

#[test]
fn test3() {
    let input = read_file("day14/input_test_part1_test3").unwrap();
    assert_eq!(solve(&input), 9251071085);
}

#[test]
fn test4() {
    let input = read_file("day14/input_test_part1_test4").unwrap();
    assert_eq!(solve(&input), 5941429882);
}
