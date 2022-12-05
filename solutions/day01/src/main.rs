fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn get_sums_iter(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.split("\n\n").map(|section| {
        section
            .lines()
            .map(|cal_str| cal_str.parse::<u32>().unwrap())
            .sum()
    })
}

fn get_top_three(mut top_three: [u32; 3], s: u32) -> [u32; 3] {
    if s > top_three[0] {
        top_three[2] = top_three[1];
        top_three[1] = top_three[0];
        top_three[0] = s;
    } else if s > top_three[1] {
        top_three[2] = top_three[1];
        top_three[1] = s;
    } else if s > top_three[2] {
        top_three[2] = s;
    }
    top_three
}

fn problem1(input: &str) -> u32 {
    get_sums_iter(input).max().unwrap()
}

fn problem2(input: &str) -> u32 {
    get_sums_iter(input)
        .fold([0; 3], get_top_three)
        .into_iter()
        .sum()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 24000);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 45000);
}
