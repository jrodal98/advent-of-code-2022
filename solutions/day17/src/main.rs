use std::collections::HashMap;

use chamber::{Chamber, Snapshot};

const PART1_DROPS: usize = 2022;
const PART2_DROPS: usize = 1_000_000_000_000;

pub mod chamber;
pub mod coordinate;
pub mod rock;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    drop_rocks(input, PART1_DROPS)
}

fn problem2(input: &str) -> usize {
    drop_rocks(input, PART2_DROPS)
}

fn drop_rocks(input: &str, num_rocks: usize) -> usize {
    let mut chamber = Chamber::new();
    let mut dir_iter = input.trim_end().chars().cycle();
    let mut cache: HashMap<Snapshot, usize> = HashMap::new();
    let mut heights: Vec<usize> = Vec::new();
    for r in 0..num_rocks {
        heights.push(chamber.top());
        if let Some(snapshot_r) = cache.insert(chamber.get_snapshot(), r) {
            let current_height = *heights.last().unwrap();
            let snapshot_height = heights[snapshot_r];
            let cycle_size = r - snapshot_r;

            let complete_cycles = (num_rocks - r) / cycle_size;
            let r_remaining = (num_rocks - r) % cycle_size;

            return current_height
                + (current_height - snapshot_height) * complete_cycles
                + (heights[snapshot_r + r_remaining] - snapshot_height);
        }
        chamber.start_dropping_rock(r);
        loop {
            match dir_iter.next().unwrap() {
                '>' => chamber.move_rock_right(),
                '<' => chamber.move_rock_left(),
                _ => unreachable!("Invalid input!"),
            }
            if !chamber.move_rock_down() {
                break;
            }
        }
    }
    *heights.last().unwrap()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 3068);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 1514285714288);
}
