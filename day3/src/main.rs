use std::collections::HashSet;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

// I could probably do some ascii mathz, but I feel like
// the compiler could optimize a match statement better?
fn char_to_priority(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => unreachable!("Invalid input"),
    }
}

fn problem1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| -> u32 {
            let rucksack_size = line.len() / 2;
            let rucksack1: HashSet<char> = line[..rucksack_size].chars().collect();
            let rucksack2: HashSet<char> = line[rucksack_size..].chars().collect();

            let duplicate = rucksack1.intersection(&rucksack2).collect::<Vec<&char>>()[0];
            char_to_priority(*duplicate)
        })
        .sum()
}

fn problem2(input: &str) -> u32 {
    input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            if let &[sack1, sack2, sack3] = chunk {
                let set1: HashSet<char> = sack1.chars().collect();
                let set2: HashSet<char> = sack2.chars().collect();
                let set3: HashSet<char> = sack3.chars().collect();

                for c in set1.intersection(&set2) {
                    if set3.contains(c) {
                        return char_to_priority(*c);
                    }
                }
                panic!("Invalid input - no duplicate char")
            } else {
                panic!("invalid input - malformed chunks")
            }
        })
        .sum()
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
