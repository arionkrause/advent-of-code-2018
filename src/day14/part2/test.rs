use crate::day14::part2::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day14/input_test_part2_test1").unwrap();
    assert_eq!(solve(&input), 9);
}

#[test]
fn test2() {
    let input = read_file("day14/input_test_part2_test2").unwrap();
    assert_eq!(solve(&input), 5);
}

#[test]
fn test3() {
    let input = read_file("day14/input_test_part2_test3").unwrap();
    assert_eq!(solve(&input), 18);
}

#[test]
fn test4() {
    let input = read_file("day14/input_test_part2_test4").unwrap();
    assert_eq!(solve(&input), 2018);
}
