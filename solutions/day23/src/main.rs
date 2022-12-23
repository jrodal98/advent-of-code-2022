pub mod coordinate;
pub mod grid;

use crate::grid::Grid;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    const NUM_ROUNDS: usize = 10;
    input
        .parse::<Grid>()
        .unwrap()
        .compute_ground_tiles(NUM_ROUNDS)
}

fn problem2(input: &str) -> usize {
    input
        .parse::<Grid>()
        .unwrap()
        .num_rounds_until_no_movement()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        let res = problem1(input);
        assert_eq!(res, 110);
    }

    #[test]
    fn test_problem2() {
        let input = include_str!("../data/sample.txt");
        let res = problem2(input);
        assert_eq!(res, 20);
    }
}
