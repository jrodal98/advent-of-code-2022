use chamber::Chamber;

pub mod chamber;
pub mod coordinate;
pub mod rock;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> usize {
    let mut chamber = Chamber::new();
    let mut dir_iter = input.trim_end().chars().cycle();
    for r in 0..2022 {
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
    chamber.top()
}

fn problem2(input: &str) -> usize {
    unimplemented!()
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
    assert_eq!(res, 0);
}
