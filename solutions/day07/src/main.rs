use std::{cell::RefCell, collections::HashMap, rc::Rc};

const FILESYSTEM_SIZE: u32 = 70000000;
const UNUSED_SPACE_NEEDED: u32 = 30000000;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Debug, Clone)]
struct File<'a> {
    name: &'a str,
    size: u32,
}

#[derive(Debug, Clone)]
struct Directory<'a> {
    name: &'a str,
    directories: HashMap<&'a str, Rc<RefCell<Directory<'a>>>>,
    files: Vec<File<'a>>,
}

impl<'a> File<'a> {
    fn name(&self) -> &str {
        self.name
    }

    fn size(&self) -> u32 {
        self.size
    }
}

impl<'a> Directory<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            directories: HashMap::new(),
            files: vec![],
        }
    }
    fn name(&self) -> &str {
        self.name
    }

    fn directory_sizes(&self) -> u32 {
        self.directories.values().map(|d| d.borrow().size()).sum()
    }

    fn file_sizes(&self) -> u32 {
        self.files.iter().map(|f| f.size()).sum()
    }

    fn size(&self) -> u32 {
        self.directory_sizes() + self.file_sizes()
    }

    fn sum_under_n(&self, n: u32) -> u32 {
        self.recursive_directory_sizes()
            .into_iter()
            .filter(|s| s <= &n)
            .sum()
    }

    fn sizes_over_n(&self, n: u32) -> Vec<u32> {
        self.recursive_directory_sizes()
            .into_iter()
            .filter(|s| s >= &n)
            .collect()
    }

    fn recursive_directory_sizes(&self) -> Vec<u32> {
        let mut sizes: Vec<u32> = self
            .directories
            .values()
            .flat_map(|d| d.borrow().recursive_directory_sizes())
            .collect();

        sizes.push(self.size());
        sizes
    }
}

fn get_root_dir_from_input<'a>(input: &'a str) -> Rc<RefCell<Directory<'a>>> {
    let root_dir = Rc::new(RefCell::new(Directory::new("/")));
    let mut directories: Vec<Rc<RefCell<Directory>>> = vec![];

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        match line.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", ".."] => {
                directories.pop();
            }
            ["$", "cd", "/"] => {
                directories.clear();
                directories.push(root_dir.clone());
            }
            ["$", "cd", x] => {
                let directory = directories.last().unwrap().clone();
                let mut y = directory.borrow_mut();
                let cd_dir = y
                    .directories
                    .entry(x)
                    .or_insert(Rc::new(RefCell::new(Directory::new(x))));
                directories.push(cd_dir.clone());
            }
            ["$", "ls"] => {}
            ["dir", d] => {
                let directory = directories.last_mut().unwrap();
                directory
                    .borrow_mut()
                    .directories
                    .insert(d, Rc::new(RefCell::new(Directory::new(d))));
            }
            [size, name] => {
                let directory = directories.last_mut().unwrap();
                let file = File {
                    name,
                    size: size.parse().unwrap(),
                };
                directory.borrow_mut().files.push(file);
            }
            _ => unreachable!(),
        }
    }

    root_dir
}

fn problem1(input: &str) -> u32 {
    get_root_dir_from_input(input).borrow().sum_under_n(100000)
}

fn problem2(input: &str) -> u32 {
    let root_dir_cell = get_root_dir_from_input(input);
    let root_dir = root_dir_cell.borrow();
    let space_consumed = FILESYSTEM_SIZE - root_dir.size();
    let space_to_free = UNUSED_SPACE_NEEDED - space_consumed;
    root_dir
        .sizes_over_n(space_to_free)
        .into_iter()
        .min()
        .unwrap()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 95437);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 24933642);
}
