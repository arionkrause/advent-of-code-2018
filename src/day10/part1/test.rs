use crate::day10::part1::*;
use crate::io::*;

#[test]
fn test1() {
    let input = read_file("day10/input_test_part1_test1").unwrap();
    assert_eq!(solve(&input), "#...#..###\n#...#...#.\n#...#...#.\n#####...#.\n#...#...#.\n#...#...#.\n#...#...#.\n#...#..###\n");
}
