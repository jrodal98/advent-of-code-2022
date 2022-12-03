fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn score_round(line: &str) -> u32 {
    match line.split_once(" ").unwrap() {
        ("A", "X") => 1 + 3,
        ("B", "X") => 1 + 0,
        ("C", "X") => 1 + 6,
        ("A", "Y") => 2 + 6,
        ("B", "Y") => 2 + 3,
        ("C", "Y") => 2 + 0,
        ("A", "Z") => 3 + 0,
        ("B", "Z") => 3 + 6,
        ("C", "Z") => 3 + 3,
        _ => unreachable!("Invalid input"),
    }
}

fn problem1(input: &str) -> u32 {
    input.lines().map(score_round).sum()
}

fn problem2(input: &str) -> u32 {
    0
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
    assert_eq!(res, 0);
}
