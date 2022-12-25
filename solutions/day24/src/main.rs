pub mod blizzard;
pub mod coordinate;
pub mod direction;
pub mod grid;

use grid::Grid;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    input.parse::<Grid>().unwrap().find_fastest_exit() as u32
}

fn problem2(input: &str) -> u32 {
    input.parse::<Grid>().unwrap().round_trip_for_snacks() as u32
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 18);
}

// part 2 does not work on sample input for some reason
// even though it works on puzzle input

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 54);
}
