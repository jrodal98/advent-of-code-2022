fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn get_stacks(stacks_str: &str) -> Vec<Vec<char>> {
    let num_columns = (stacks_str.lines().next().unwrap().chars().count() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![vec![]; num_columns];

    for line in stacks_str.lines() {
        line.as_bytes()
            .chunks(4)
            .enumerate()
            .filter(|(_, chunk)| (chunk[1] as char).is_alphabetic())
            .for_each(|(i, chunk)| stacks[i].insert(0, chunk[1] as char));
    }

    stacks
}

fn line_to_move_instruction(line: &str) -> (usize, usize, usize) {
    let tokens: Vec<usize> = line
        .split_whitespace()
        .filter_map(|word| word.parse::<usize>().ok())
        .collect();

    (tokens[0], tokens[1] - 1, tokens[2] - 1)
}

fn solve_problem(input: &str, preserve_order: bool) -> String {
    let (stacks_str, instruction_str) = input.split_once("\n\n").unwrap();

    let mut stacks = get_stacks(stacks_str);

    for line in instruction_str.lines() {
        let (num_boxes, from, to) = line_to_move_instruction(line);
        let mut boxes_to_push = Vec::new();

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

    stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop())
        .collect()
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
