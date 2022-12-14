const CRT_SCREEN_SIZE: usize = 240;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2:\n{}", problem2(input));
}

enum Instruction {
    NOOP,
    ADDX(isize),
}

use std::str::FromStr;

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ") {
            Some((_instruction_str, x)) => Ok(Instruction::ADDX(x.parse::<isize>().unwrap())),
            None => Ok(Instruction::NOOP),
        }
    }
}

struct Cpu {
    x: isize,
    x_history: [isize; CRT_SCREEN_SIZE],
    image: [char; CRT_SCREEN_SIZE],
    pixel_to_draw: usize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            x_history: [1; CRT_SCREEN_SIZE],
            image: ['.'; CRT_SCREEN_SIZE],
            pixel_to_draw: 0,
        }
    }

    fn draw_sprite(&mut self) {
        if (self.x - 1..=self.x + 1).contains(&(self.pixel_to_draw as isize % 40)) {
            self.image[self.pixel_to_draw] = '#'
        }
        self.pixel_to_draw += 1;
    }
    fn process_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NOOP => {
                self.draw_sprite();
                self.record_x();
            }
            Instruction::ADDX(x) => {
                self.draw_sprite();
                self.record_x();
                self.draw_sprite();
                self.x += x;
                self.record_x()
            }
        }
    }

    fn record_x(&mut self) {
        self.x_history[self.pixel_to_draw - 1] = self.x;
    }

    fn process_program(&mut self, input: &str) {
        input
            .lines()
            .filter_map(|line| line.parse::<Instruction>().ok())
            .for_each(|instruction| self.process_instruction(instruction));
    }
}

fn problem1(input: &str) -> isize {
    let mut cpu = Cpu::new();
    cpu.process_program(input);
    cpu.x_history
        .iter()
        .skip(18)
        .step_by(40)
        .enumerate()
        .map(|(i, x)| (i as isize * 40 + 20) * x)
        .sum()
}

fn problem2(input: &str) -> String {
    let mut cpu = Cpu::new();
    cpu.process_program(input);

    cpu.image
        .chunks(40)
        .map(|slice| slice.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 13140);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let expected_res = include_str!("../data/part2_sample.txt").trim().to_string();
    let res = problem2(input);
    assert_eq!(res, expected_res);
}
