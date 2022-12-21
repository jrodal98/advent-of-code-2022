use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

struct Monkey {
    operation: Operation,
}

impl Default for Monkey {
    fn default() -> Self {
        Self {
            operation: Operation::Panic,
        }
    }
}

impl Monkey {
    fn evaluate(&self) -> isize {
        self.operation.evaluate()
    }
}

enum Operation {
    Yell(isize),
    Add(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>),
    Subtract(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>),
    Multiply(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>),
    Divide(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>),
    Panic,
}

impl Operation {
    fn evaluate(&self) -> isize {
        match &self {
            Operation::Yell(v) => *v,
            Operation::Add(m1, m2) => m1.borrow().evaluate() + m2.borrow().evaluate(),
            Operation::Subtract(m1, m2) => m1.borrow().evaluate() - m2.borrow().evaluate(),
            Operation::Multiply(m1, m2) => m1.borrow().evaluate() * m2.borrow().evaluate(),
            Operation::Divide(m1, m2) => m1.borrow().evaluate() / m2.borrow().evaluate(),
            Operation::Panic => panic!("This monkey has no operation"),
        }
    }
}

fn problem1(input: &str) -> isize {
    let mut monkeys: HashMap<&str, Rc<RefCell<Monkey>>> = HashMap::new();
    for line in input.lines() {
        let (name, operation_str) = line.split_once(": ").unwrap();
        let monkey = monkeys.entry(name).or_default().clone();
        let operation = if let Some(num) = operation_str.parse::<isize>().ok() {
            Operation::Yell(num)
        } else {
            let monkey1 = monkeys.entry(&operation_str[..4]).or_default().clone();
            let monkey2 = monkeys.entry(&operation_str[7..]).or_default().clone();
            match operation_str.chars().nth(5).unwrap() {
                '+' => Operation::Add(monkey1, monkey2),
                '-' => Operation::Subtract(monkey1, monkey2),
                '*' => Operation::Multiply(monkey1, monkey2),
                '/' => Operation::Divide(monkey1, monkey2),
                _ => unreachable!(),
            }
        };

        monkey.borrow_mut().operation = operation;
    }
    let res = monkeys.get("root").unwrap().borrow().evaluate();
    res
}

fn problem2(input: &str) -> isize {
    let mut monkeys: HashMap<&str, Rc<RefCell<Monkey>>> = HashMap::new();
    for line in input.lines() {
        let (name, operation_str) = line.split_once(": ").unwrap();
        let monkey = monkeys.entry(name).or_default().clone();
        let operation = if let Some(num) = operation_str.parse::<isize>().ok() {
            Operation::Yell(num)
        } else {
            let monkey1 = monkeys.entry(&operation_str[..4]).or_default().clone();
            let monkey2 = monkeys.entry(&operation_str[7..]).or_default().clone();
            match operation_str.chars().nth(5).unwrap() {
                '+' => Operation::Add(monkey1, monkey2),
                '-' => Operation::Subtract(monkey1, monkey2),
                '*' => Operation::Multiply(monkey1, monkey2),
                '/' => Operation::Divide(monkey1, monkey2),
                _ => unreachable!(),
            }
        };

        monkey.borrow_mut().operation = operation;
    }
    let res = monkeys.get("root").unwrap().borrow().evaluate();
    res
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 152);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 0);
}
