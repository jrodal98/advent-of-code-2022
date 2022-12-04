fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOSE: u32 = 0;
const TIE: u32 = 3;
const WIN: u32 = 6;

fn problem1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match line.split_once(" ").unwrap() {
            ("A", "X") => ROCK + TIE,
            ("B", "X") => ROCK + LOSE,
            ("C", "X") => ROCK + WIN,
            ("A", "Y") => PAPER + WIN,
            ("B", "Y") => PAPER + TIE,
            ("C", "Y") => PAPER + LOSE,
            ("A", "Z") => SCISSORS + LOSE,
            ("B", "Z") => SCISSORS + WIN,
            ("C", "Z") => SCISSORS + TIE,
            _ => unreachable!("Invalid input"),
        })
        .sum()
}

fn problem2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match line.split_once(" ").unwrap() {
            ("A", "X") => LOSE + SCISSORS,
            ("B", "X") => LOSE + ROCK,
            ("C", "X") => LOSE + PAPER,
            ("A", "Y") => TIE + ROCK,
            ("B", "Y") => TIE + PAPER,
            ("C", "Y") => TIE + SCISSORS,
            ("A", "Z") => WIN + PAPER,
            ("B", "Z") => WIN + SCISSORS,
            ("C", "Z") => WIN + ROCK,
            _ => unreachable!("Invalid input"),
        })
        .sum()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 15);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 12);
}
