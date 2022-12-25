use snafu::Snafu;

pub mod snafu;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
}

fn problem1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Snafu>().unwrap())
        .sum::<Snafu>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem1() {
        let input = include_str!("../data/sample.txt");
        let res = problem1(input);
        assert_eq!(res, "2=-1=0".to_string());
    }
}
