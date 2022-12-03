mod prority;

use std::collections::HashSet;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let (sack1_str, sack2) = line.split_at(line.len() / 2);
        let sack1: HashSet<char> = sack1_str.chars().collect();

        for c in sack2.chars() {
            if sack1.contains(&c) {
                return acc + prority::char_to_priority(c);
            }
        }
        unreachable!("Invalid input - no duplicate char")
    })
}

fn problem2(input: &str) -> u32 {
    let mut lines = input.lines().peekable();
    let mut sum = 0;
    while lines.peek().is_some() {
        let set1: HashSet<char> = lines.next().unwrap().chars().collect();
        let set2: HashSet<char> = lines.next().unwrap().chars().collect();
        let set3: HashSet<char> = lines.next().unwrap().chars().collect();
        for c in set1.intersection(&set2) {
            if set3.contains(c) {
                sum += prority::char_to_priority(*c);
                break;
            }
        }
    }
    sum
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 157);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 70);
}
