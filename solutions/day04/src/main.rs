fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.split_once(",").unwrap())
        .map(|(s1, s2)| {
            let (s1_min_str, s1_max_str) = s1.split_once("-").unwrap();
            let (s2_min_str, s2_max_str) = s2.split_once("-").unwrap();

            ((s1_min_str.parse::<u32>().unwrap() <= s2_min_str.parse().unwrap()
                && s1_max_str.parse::<u32>().unwrap() >= s2_max_str.parse().unwrap())
                || (s2_min_str.parse::<u32>().unwrap() <= s1_min_str.parse().unwrap()
                    && s2_max_str.parse::<u32>().unwrap() >= s1_max_str.parse().unwrap()))
                as u32
        })
        .sum()
}

fn problem2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.split_once(",").unwrap())
        .map(|(s1, s2)| {
            let (s1_min_str, s1_max_str) = s1.split_once("-").unwrap();
            let (s2_min_str, s2_max_str) = s2.split_once("-").unwrap();

            ((s1_max_str.parse::<u32>().unwrap() >= s2_min_str.parse().unwrap()
                && s1_max_str.parse::<u32>().unwrap() <= s2_max_str.parse().unwrap())
                || (s2_max_str.parse::<u32>().unwrap() >= s1_min_str.parse().unwrap()
                    && s2_max_str.parse::<u32>().unwrap() <= s1_max_str.parse().unwrap()))
                as u32
        })
        .sum()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 2);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 4);
}
