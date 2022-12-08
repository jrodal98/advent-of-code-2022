fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

struct Grid {
    rows: Vec<Vec<u8>>,
    // yes, this is memory inefficient
    // no, I do not care
    cols: Vec<Vec<u8>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let num_cols = input.lines().next().unwrap().chars().count();
        let mut cols: Vec<Vec<u8>> = vec![Vec::new(); num_cols];

        let rows: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let val = c.to_digit(10).unwrap() as u8;
                        cols[i].push(val);
                        val
                    })
                    .collect()
            })
            .collect();

        Self { rows, cols }
    }

    fn num_visible_trees(&self) -> u32 {
        let num_rows = self.rows.len();
        let num_columns = self.rows[0].len();
        let mut num_visible_trees = (num_rows * 2 + num_columns * 2 - 4) as u32;

        for i in 1..num_rows - 1 {
            for j in 1..num_columns - 1 {
                num_visible_trees += self.is_tree_visible(i, j) as u32;
            }
        }

        num_visible_trees
    }

    fn max_scenic_score(&self) -> u32 {
        let num_rows = self.rows.len();
        let num_columns = self.rows[0].len();
        let mut max_score = 0;

        for i in 1..num_rows - 1 {
            for j in 1..num_columns - 1 {
                let score = self.tree_scenic_score(i, j) as u32;
                if score > max_score {
                    max_score = score;
                }
            }
        }

        max_score
    }

    fn is_tree_visible(&self, i: usize, j: usize) -> bool {
        let tree = &self.rows[i][j];

        self.is_tree_visible_left(tree, i, j)
            || self.is_tree_visible_right(tree, i, j)
            || self.is_tree_visible_up(tree, i, j)
            || self.is_tree_visible_down(tree, i, j)
    }

    fn is_tree_visible_one_direction(tree: &u8, other_trees: &[u8]) -> bool {
        other_trees.iter().all(|t| t < tree)
    }

    fn is_tree_visible_left(&self, tree: &u8, i: usize, j: usize) -> bool {
        Self::is_tree_visible_one_direction(tree, &self.rows[i][..j])
    }

    fn is_tree_visible_right(&self, tree: &u8, i: usize, j: usize) -> bool {
        Self::is_tree_visible_one_direction(tree, &self.rows[i][j + 1..])
    }

    fn is_tree_visible_down(&self, tree: &u8, i: usize, j: usize) -> bool {
        Self::is_tree_visible_one_direction(tree, &self.cols[j][i + 1..])
    }
    fn is_tree_visible_up(&self, tree: &u8, i: usize, j: usize) -> bool {
        Self::is_tree_visible_one_direction(tree, &self.cols[j][..i])
    }

    fn tree_scenic_score(&self, i: usize, j: usize) -> u32 {
        let tree = &self.rows[i][j];
        self.scenic_score_left(tree, i, j)
            * self.scenic_score_right(tree, i, j)
            * self.scenic_score_down(tree, i, j)
            * self.scenic_score_up(tree, i, j)
    }

    fn scenic_score_one_direction(tree: &u8, other_trees: &[u8], rev: bool) -> u32 {
        // if it's zero, add 1
        // if it's less than the length, add 1
        let c = if rev {
            other_trees.iter().rev().take_while(|t| *t < tree).count()
        } else {
            other_trees.iter().take_while(|t| *t < tree).count()
        };

        (c + 1).min(other_trees.len()) as u32
    }

    fn scenic_score_left(&self, tree: &u8, i: usize, j: usize) -> u32 {
        Self::scenic_score_one_direction(tree, &self.rows[i][..j], true)
    }

    fn scenic_score_right(&self, tree: &u8, i: usize, j: usize) -> u32 {
        Self::scenic_score_one_direction(tree, &self.rows[i][j + 1..], false)
    }

    fn scenic_score_down(&self, tree: &u8, i: usize, j: usize) -> u32 {
        Self::scenic_score_one_direction(tree, &self.cols[j][i + 1..], false)
    }
    fn scenic_score_up(&self, tree: &u8, i: usize, j: usize) -> u32 {
        Self::scenic_score_one_direction(tree, &self.cols[j][..i], true)
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
