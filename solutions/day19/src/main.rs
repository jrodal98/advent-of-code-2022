use game::Game;

pub mod factory;
pub mod game;

use rayon::prelude::*;

fn main() {
    let input = include_str!("../data/input.txt");
    // let input = "Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 11 clay. Each geode robot costs 4 ore and 7 obsidian.";
    // let input = include_str!("../data/sample.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u16 {
    // answer is 1589
    input
        .lines()
        .par_bridge()
        .map(|line| line.parse::<Game>().unwrap().quality_level())
        .sum()
}

fn problem2(input: &str) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_problem1_simple() {
    //     let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    //     let res = problem1(input);
    //     assert_eq!(res, 9);
    // }

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        let res = problem1(input);
        assert_eq!(res, 33);
    }
    //
    // #[test]
    // fn test_problem2() {
    //     let input = include_str!("../data/sample.txt");
    //     let res = problem2(input);
    //     assert_eq!(res, 0);
    // }
}
