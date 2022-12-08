use std::ops::Add;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

struct Grid<'a> {
    rows: Vec<&'a [u8]>,
    num_rows: usize,
    num_cols: usize,
}

impl<'a> Grid<'a> {
    fn new(input: &'a str) -> Self {
        let rows: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
        let num_rows = rows.len();
        let num_cols = match rows.get(0) {
            Some(r) => r.len(),
            None => 0,
        };

        Self {
            rows,
            num_rows,
            num_cols,
        }
    }

    fn num_visible_trees(&self) -> u32 {
        let mut num_visible_trees = (self.num_rows * 2 + self.num_cols * 2 - 4) as u32;

        for i in 1..self.num_rows - 1 {
            for j in 1..self.num_cols - 1 {
                num_visible_trees += self.is_tree_visible(i, j) as u32;
            }
        }

        num_visible_trees
    }

    fn max_scenic_score(&self) -> u32 {
        let num_rows = self.rows.len();
        let num_columns = self.rows[0].len();
        let mut max_score = 0;

        for i in 0..num_rows {
            for j in 0..num_columns {
                let score = self.tree_scenic_score(i, j) as u32;
                if score > max_score {
                    max_score = score;
                }
            }
        }

        max_score
    }

    fn is_tree_visible(&self, i: usize, j: usize) -> bool {
        let tree = self.rows[i][j];

        (0..j).all(|v| self.rows[i][v] < tree)
            || (j + 1..self.num_cols).all(|v| self.rows[i][v] < tree)
            || (i + 1..self.num_rows).all(|v| self.rows[v][j] < tree)
            || (0..i).all(|v| self.rows[v][j] < tree)
    }

    fn tree_scenic_score(&self, i: usize, j: usize) -> u32 {
        let tree = self.rows[i][j];

        let score = (0..j)
            .rev()
            .take_while(|&v| self.rows[i][v] < tree)
            .count()
            .add(1)
            .min(j)
            * (j + 1..self.num_cols)
                .take_while(|&v| self.rows[i][v] < tree)
                .count()
                .add(1)
                .min(self.num_cols - j - 1)
            * (i + 1..self.num_rows)
                .take_while(|&v| self.rows[v][j] < tree)
                .count()
                .add(1)
                .min(self.num_rows - i - 1)
            * (0..i)
                .rev()
                .take_while(|&v| self.rows[v][j] < tree)
                .count()
                .add(1)
                .min(i);

        score as u32
    }
}

fn problem1(input: &str) -> u32 {
    Grid::new(input).num_visible_trees()
}

fn problem2(input: &str) -> u32 {
    Grid::new(input).max_scenic_score()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 21);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 8);
}
