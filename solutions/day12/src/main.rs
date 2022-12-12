use std::collections::VecDeque;

const S: u8 = 'S' as u8;
const E: u8 = 'E' as u8;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

fn transform_height(height: u8) -> u8 {
    if height == E {
        'z' as u8
    } else if height == S {
        'a' as u8
    } else {
        height
    }
}

fn check_direction(
    x: usize,
    y: usize,
    l: u32,
    max_height: u8,
    grid: &Vec<&[u8]>,
    visited: &mut Vec<Vec<bool>>,
    queue: &mut VecDeque<(usize, usize, u32)>,
) {
    if visited[x][y] {
        return;
    }

    let height = transform_height(grid[x][y]);

    if height <= max_height {
        queue.push_back((x, y, l + 1));
        visited[x][y] = true;
    }
}

fn shortest_path(i: usize, j: usize, grid: &Vec<&[u8]>) -> u32 {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut queue: VecDeque<(usize, usize, u32)> = VecDeque::from(vec![(i, j, 0)]);

    while let Some((r, c, l)) = queue.pop_front() {
        let height = grid[r][c];

        if height == E {
            return l;
        }

        let max_height = transform_height(height) + 1;

        if r < grid.len() - 1 {
            check_direction(r + 1, c, l, max_height, grid, &mut visited, &mut queue);
        }

        if c > 0 {
            check_direction(r, c - 1, l, max_height, grid, &mut visited, &mut queue);
        }

        if c < grid[r].len() - 1 {
            check_direction(r, c + 1, l, max_height, grid, &mut visited, &mut queue);
        }

        if r > 0 {
            check_direction(r - 1, c, l, max_height, grid, &mut visited, &mut queue);
        }
    }

    panic!("Could not find target E")
}

fn problem1(input: &str) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&b| b == S) {
            return shortest_path(i, j, &grid);
        }
    }
    panic!("Could not find start S");
}

fn problem2(input: &str) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let mut paths = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&b| b == S || b == 'a' as u8) {
            paths.push(shortest_path(i, j, &grid));
        }
    }
    paths.into_iter().min().expect("No path to E")
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 31);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 29);
}
