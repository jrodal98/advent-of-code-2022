extern crate num;
use num::Integer;
use std::{cell::RefCell, str::FromStr};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

enum Operation {
    Add(Option<u64>),
    Multiply(Option<u64>),
}

enum Modifer {
    Worried(u64),
    NotWorried(u64),
}

impl Modifer {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Self::Worried(n) => value / n,
            Self::NotWorried(n) => value % n,
        }
    }
}

impl Operation {
    fn apply(&self, value: u64) -> u64 {
        match self {
            Operation::Add(Some(n)) => value + n,
            Operation::Add(None) => value + value,
            Operation::Multiply(Some(n)) => value * n,
            Operation::Multiply(None) => value * value,
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.trim().splitn(6, " ").skip(4);
        let operand = splitted.next().ok_or("No operand found")?;
        let num = splitted.next().ok_or("No num found")?.parse::<u64>().ok();

        match operand {
            "+" => Ok(Self::Add(num)),
            "*" => Ok(Self::Multiply(num)),
            o => Err(format!("Unsupported operand {}", o)),
        }
    }
}

#[derive(Default)]
struct Monkey<'a> {
    items: Vec<u64>,
    operation: Option<Operation>,
    test: u64,
    true_monkey: Option<&'a RefCell<Monkey<'a>>>,
    false_monkey: Option<&'a RefCell<Monkey<'a>>>,
    num_inspections: usize,
}

impl<'a> Monkey<'a> {
    fn update(
        &mut self,
        items: Vec<u64>,
        operation: Operation,
        test: u64,
        true_monkey: &'a RefCell<Monkey<'a>>,
        false_monkey: &'a RefCell<Monkey<'a>>,
    ) {
        self.items = items;
        self.operation = Some(operation);
        self.test = test;
        self.true_monkey = Some(true_monkey);
        self.false_monkey = Some(false_monkey);
    }
    fn receive_item(&mut self, item: u64) {
        self.items.push(item)
    }
    fn throw_items(&mut self, modifier: &Modifer) {
        let operation = self.operation.as_ref().unwrap();
        let mut true_monkey = self.true_monkey.as_ref().unwrap().borrow_mut();
        let mut false_monkey = self.false_monkey.as_ref().unwrap().borrow_mut();

        for item in self.items.iter() {
            let new_item = modifier.apply(operation.apply(*item));
            if new_item % self.test == 0 {
                true_monkey.receive_item(new_item)
            } else {
                false_monkey.receive_item(new_item)
            };
        }
        self.num_inspections += self.items.len();
        self.items.clear();
    }
}

fn extract_last_number(line: &str) -> u64 {
    line.split_whitespace()
        .last()
        .map(|num| num.parse::<u64>().unwrap())
        .unwrap()
}

fn solve_problem(input: &str, rounds: u64, worried: bool) -> u64 {
    let monkey_sections = input.split("\n\n");
    let monkeys: Vec<RefCell<Monkey>> = (0..monkey_sections.clone().count())
        .map(|_| RefCell::new(Monkey::default()))
        .collect();

    for (monkey_num, monkey_section) in monkey_sections.enumerate() {
        let mut lines = monkey_section.lines().skip(1);

        let (_, items_str) = lines.next().unwrap().trim().split_at(16);
        let items: Vec<u64> = items_str
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect();

        let operation = lines.next().unwrap().parse::<Operation>().unwrap();

        let test = extract_last_number(lines.next().unwrap());

        let true_monkey_num = extract_last_number(lines.next().unwrap()) as usize;
        let false_monkey_num = extract_last_number(lines.next().unwrap()) as usize;
        let true_monkey = monkeys.get(true_monkey_num).unwrap().clone();
        let false_monkey = monkeys.get(false_monkey_num).unwrap().clone();

        monkeys.get(monkey_num).unwrap().borrow_mut().update(
            items,
            operation,
            test,
            true_monkey,
            false_monkey,
        );
    }

    let modifier = if worried {
        Modifer::Worried(3)
    } else {
        let lcm = monkeys
            .iter()
            .map(|m| m.borrow().test)
            .reduce(|lcm, n| lcm.lcm(&n))
            .unwrap();
        Modifer::NotWorried(lcm)
    };

    for _round in 0..rounds {
        for monkey in monkeys.iter() {
            monkey.borrow_mut().throw_items(&modifier);
        }
    }

    let mut monkey_inspections: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.borrow().num_inspections)
        .collect();

    monkey_inspections.sort();

    monkey_inspections
        .into_iter()
        .rev()
        .take(2)
        .reduce(|v1, v2| v1 * v2)
        .unwrap() as u64
}

fn problem1(input: &str) -> u64 {
    solve_problem(input, 20, true)
}

fn problem2(input: &str) -> u64 {
    solve_problem(input, 10000, false)
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 10605);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 2713310158);
}
