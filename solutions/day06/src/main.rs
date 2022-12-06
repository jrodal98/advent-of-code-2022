use std::collections::HashSet;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn solve_problem(input: &str, num_distinct_chars: usize) -> usize {
    input
        .as_bytes()
        .windows(num_distinct_chars)
        .enumerate()
        .filter(|(_, window)| window.iter().collect::<HashSet<_>>().len() == num_distinct_chars)
        .next()
        .unwrap()
        .0
        + num_distinct_chars
}

fn problem1(input: &str) -> usize {
    solve_problem(input, 4)
}

fn problem2(input: &str) -> usize {
    solve_problem(input, 14)
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 7);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 19);
}
