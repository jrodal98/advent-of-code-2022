fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn mix(input: &str, key: isize, n: usize) -> isize {
    let values: Vec<isize> = input
        .lines()
        .map(|l| l.parse::<isize>().unwrap() * key)
        .collect();
    let num_vals = values.len() as isize;
    let mut val_proxies: Vec<usize> = (0..num_vals as usize).collect();
    let mut positions: Vec<usize> = (0..num_vals as usize).collect();
    for _ in 0..n {
        for (i, v) in values.iter().enumerate() {
            if *v == 0 {
                continue;
            }

            let current_pos = positions[i] as isize;
            let mut new_position = (current_pos + v).rem_euclid(num_vals - 1);
            if new_position == 0 {
                new_position = num_vals - 1
            } else if new_position == num_vals - 1 {
                new_position = 0;
            }

            let translation = new_position - current_pos;

            if translation > 0 {
                for j in positions[i]..positions[i] + translation as usize {
                    positions[val_proxies[j]] = j + 1;
                    positions[val_proxies[j + 1]] = j;
                    val_proxies.swap(j, j + 1);
                }
            } else if translation < 0 {
                let base = positions[i];
                for j in 0..-translation as usize {
                    let x = base - j;
                    let y = base - j - 1;
                    positions[val_proxies[x]] = y;
                    positions[val_proxies[y]] = x;
                    val_proxies.swap(x, y);
                }
            }
        }
    }

    let zero_pos = positions
        .iter()
        .zip(values.iter())
        .find(|(_, &v)| v == 0)
        .unwrap()
        .0;

    let pos_1000 = (zero_pos + 1000) % num_vals as usize;
    let pos_2000 = (zero_pos + 2000) % num_vals as usize;
    let pos_3000 = (zero_pos + 3000) % num_vals as usize;

    values[val_proxies[pos_1000]] + values[val_proxies[pos_2000]] + values[val_proxies[pos_3000]]
}

fn problem1(input: &str) -> isize {
    mix(input, 1, 1)
}

fn problem2(input: &str) -> isize {
    mix(input, 811589153, 10)
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 3);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 1623178306);
}
