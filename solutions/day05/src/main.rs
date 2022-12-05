use std::{iter::Peekable, str::Lines};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn get_stacks(lines: &mut Peekable<Lines>) -> Vec<Vec<char>> {
    let num_columns = (lines.peek().unwrap().chars().count() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_columns];

    while !lines.peek().unwrap().trim().is_empty() {
        lines
            .next()
            .unwrap()
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .enumerate()
            .map(|(box_num, chunk)| {
                if let Some(c) = chunk.iter().filter(|c| c.is_uppercase()).next() {
                    stacks[box_num].insert(0, *c);
                }
            })
            .count();
    }

    lines.next().unwrap(); // consume empty line
    stacks
}

fn line_to_move_instruction(line: &str) -> (usize, usize, usize) {
    let tokens: Vec<usize> = line
        .split_whitespace()
        .map(|word| word.parse::<usize>())
        .filter(|word| word.is_ok())
        .map(|num| num.unwrap())
        .collect();

    (tokens[0], tokens[1] - 1, tokens[2] - 1)
}

fn stacks_to_string(stacks: Vec<Vec<char>>) -> String {
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn solve_problem(input: &str, preserve_order: bool) -> String {
    let mut lines = input.lines().peekable();
    let mut stacks = get_stacks(&mut lines);

    while lines.peek().is_some() {
        let (num_boxes, from, to) = line_to_move_instruction(lines.next().unwrap());
        let mut boxes_to_push = vec![];

        for _ in 0..num_boxes {
            let val = stacks[from].pop().unwrap();
            boxes_to_push.push(val);
        }

        if preserve_order {
            for val in boxes_to_push.into_iter().rev() {
                stacks[to].push(val);
            }
        } else {
            for val in boxes_to_push.into_iter() {
                stacks[to].push(val);
            }
        }
    }

    stacks_to_string(stacks)
}

fn problem1(input: &str) -> String {
    solve_problem(input, false)
}

fn problem2(input: &str) -> String {
    solve_problem(input, true)
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, "CMZ".to_string());
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, "MCD".to_string());
}
