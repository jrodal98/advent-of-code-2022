use game::Game;

pub mod factory;
pub mod game;

use rayon::prelude::*;

const PART2_TIME: usize = 32;

fn main() {
    // let input = include_str!("../data/input.txt");
    let input = include_str!("../data/sample.txt");
    // println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u16 {
    input
        .lines()
        .par_bridge()
        .map(|line| line.parse::<Game>().unwrap().quality_level())
        .sum()
}

fn problem2(input: &str) -> u32 {
    input
        .lines()
        .take(3)
        .par_bridge()
        .map(|line| line.parse::<Game>().unwrap().max_num_geodes(PART2_TIME))
        .reduce(|| 0, |a, b| a * b) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        let res = problem1(input);
        assert_eq!(res, 33);
    }
    //
    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample.txt");
        let res = problem2(input);
        assert_eq!(res, 62 * 56);
    }
}
