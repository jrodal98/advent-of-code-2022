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
    fn evaluate(&self) -> i128 {
        self.operation.evaluate()
    }
}

enum Operation {
    Yell(i128),
    Add(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>),
    Subtract(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>),
    Multiply(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>),
    Divide(Rc<RefCell<Monkey>>, Rc<RefCell<Monkey>>),
    Panic,
}

impl Operation {
    fn evaluate(&self) -> i128 {
        match &self {
            Operation::Yell(v) => *v,
            Operation::Add(m1, m2) => m1.borrow().evaluate() + m2.borrow().evaluate(),
            Operation::Subtract(m1, m2) => m1.borrow().evaluate() - m2.borrow().evaluate(),
            Operation::Multiply(m1, m2) => m1.borrow().evaluate() * m2.borrow().evaluate(),
            Operation::Divide(m1, m2) => m1.borrow().evaluate() / m2.borrow().evaluate(),
            Operation::Panic => panic!("This monkey has no operation"),
        }
    }

    fn compute_monkeys(&self) -> (i128, i128) {
        match &self {
            Operation::Add(m1, m2)
            | Operation::Subtract(m1, m2)
            | Operation::Multiply(m1, m2)
            | Operation::Divide(m1, m2) => (m1.borrow().evaluate(), m2.borrow().evaluate()),
            Operation::Yell(_) => panic!("Only one monkey"),
            Operation::Panic => panic!("This monkey has no operation"),
        }
    }
}

fn get_monkey_map<'a>(input: &'a str) -> HashMap<&'a str, Rc<RefCell<Monkey>>> {
    let mut monkeys: HashMap<&str, Rc<RefCell<Monkey>>> = HashMap::new();
    for line in input.lines() {
        let (name, operation_str) = line.split_once(": ").unwrap();
        let monkey = monkeys.entry(name).or_default().clone();
        let operation = if let Some(num) = operation_str.parse::<i128>().ok() {
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
    monkeys
}

fn problem1(input: &str) -> i128 {
    let monkeys = get_monkey_map(input);
    let res = monkeys.get("root").unwrap().borrow().evaluate();
    res
}

fn problem2(input: &str) -> i128 {
    let monkeys = get_monkey_map(input);
    let root_monkey = monkeys.get("root").unwrap().borrow();
    let me_monkey = monkeys.get("humn").unwrap();
    // For the puzzle input, I ran it, observed the expected value of one monkey, and then
    // adjusted the upper and lower bound manually until I got this to run fast enough to
    // solve the problem.
    // I believe I could do a binary search, but I keep integer overflowing and cannot be
    // bothered to deal with it.
    for i in 0..i128::MAX {
        me_monkey.borrow_mut().operation = Operation::Yell(i);
        let (m1, m2) = root_monkey.operation.compute_monkeys();
        if m1 == m2 {
            return i;
        }
    }
    panic!("Couldn't find number")
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
    assert_eq!(res, 301);
}
