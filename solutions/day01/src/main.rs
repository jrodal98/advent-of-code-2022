fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn problem1(input: &str) -> u32 {
    let mut max = 0;
    let mut elf_calories = 0;
    for line in input.lines().map(|s| s.trim()) {
        if line.is_empty() {
            if elf_calories > max {
                max = elf_calories;
            }
            elf_calories = 0;
        } else {
            elf_calories += str::parse::<u32>(line).unwrap();
        }
    }
    max
}

fn problem2(input: &str) -> u32 {
    let mut all_elf_calories = vec![0];
    let mut elf = 0;
    for line in input.lines().map(|s| s.trim()) {
        if line.is_empty() {
            all_elf_calories.push(0);
            elf += 1;
        } else {
            all_elf_calories[elf] += str::parse::<u32>(line).unwrap();
        }
    }

    all_elf_calories.sort();

    all_elf_calories.iter().rev().take(3).sum()
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
