fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn solve_problem(input: &str, num_distinct_chars: usize) -> usize {
    input
        .as_bytes()
        .windows(num_distinct_chars)
        .position(|window| {
            // hyperfine -w 10 -N ./target/release/day06
            // yields 2.3 ms ± 0.2 ms for below solutions
            // versus 3.4 ms ± 0.2 using a set to check equality
            window
                .iter()
                .enumerate()
                .all(|(i, c)| window.iter().skip(i + 1).find(|d| c == *d).is_none())
        })
        .map(|i| i + num_distinct_chars)
        .unwrap()
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
