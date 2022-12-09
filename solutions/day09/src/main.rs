#![feature(int_abs_diff)]

use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn sign(n: isize) -> isize {
    if n < 0 {
        -1
    } else if n > 0 {
        1
    } else {
        0
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(format!("Failed to parse direction from string: {}", s)),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn move_dir(&mut self, dir: &Direction) {
        match dir {
            &Direction::Up => self.move_x_y(0, 1),
            &Direction::Down => self.move_x_y(0, -1),
            &Direction::Left => self.move_x_y(-1, 0),
            &Direction::Right => self.move_x_y(1, 0),
        }
    }

    fn follow(&mut self, other: &Coordinate) {
        let (dx, dy) = (sign(other.x - self.x), sign(other.y - self.y));
        self.move_x_y(dx, dy);
    }

    fn move_x_y(&mut self, x: isize, y: isize) {
        self.x += x;
        self.y += y;
    }

    fn adjacent(&self, other: &Coordinate) -> bool {
        self.x.abs_diff(other.x) <= 1 && self.y.abs_diff(other.y) <= 1
    }
}

struct Simulation {
    nodes: Vec<Coordinate>,
    tail_positions: HashSet<Coordinate>,
}

impl Simulation {
    fn new(num_nodes: usize) -> Self {
        let mut tail_positions = HashSet::new();
        tail_positions.insert(Coordinate::new());
        let nodes = vec![Coordinate::new(); num_nodes];
        Self {
            nodes,
            tail_positions,
        }
    }

    fn move_once(&mut self, dir: &Direction) {
        let mut iter = self.nodes.iter_mut();

        let mut previous = iter.next().unwrap();
        previous.move_dir(dir);

        for node in iter {
            if previous.adjacent(node) {
                return;
            }
            node.follow(previous);
            previous = node;
        }

        self.tail_positions.insert(previous.clone());
    }

    fn simulate_movement(&mut self, line: &str) {
        let (dir_str, dir_num) = line.split_once(" ").unwrap();
        let dir = dir_str.parse::<Direction>().unwrap();
        let num = dir_num.parse::<usize>().unwrap();
        for _ in 0..num {
            self.move_once(&dir);
        }
    }

    fn run(&mut self, input: &str) -> u32 {
        input.lines().for_each(|line| {
            self.simulate_movement(line);
        });
        self.tail_positions.len() as u32
    }
}

fn problem1(input: &str) -> u32 {
    Simulation::new(2).run(input)
}

fn problem2(input: &str) -> u32 {
    Simulation::new(10).run(input)
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 13);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 1);
}
