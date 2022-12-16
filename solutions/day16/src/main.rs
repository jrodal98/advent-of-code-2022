use std::{cell::RefCell, collections::HashMap, rc::Rc};

const STARTING_TIME: u32 = 30;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

struct Valve {
    name: String,
    flow_rate: u32,
    neighbors: Vec<Rc<RefCell<Valve>>>,
}

impl Valve {
    fn from_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            flow_rate: 0,
            neighbors: Vec::new(),
        }
    }

    fn from_input(input: &str) -> Rc<RefCell<Valve>> {
        let mut valves: HashMap<&str, Rc<RefCell<Valve>>> = HashMap::new();
        // set name and flow rate
        for line in input.lines() {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            let valve_name = tokens[1];
            let flow_rate = tokens[4][5..].strip_suffix(";").unwrap();
            let tunnels: Vec<Rc<RefCell<Valve>>> = tokens[9..]
                .iter()
                .map(|n| n.strip_suffix(",").unwrap_or(n))
                .map(|n| {
                    valves
                        .entry(n)
                        .or_insert(Rc::new(RefCell::new(Valve::from_name(n))))
                        .clone()
                })
                .collect();

            let valve = valves
                .entry(valve_name)
                .or_insert(Rc::new(RefCell::new(Valve::from_name(valve_name))));
            valve.borrow_mut().flow_rate = flow_rate.parse().unwrap();
            valve.borrow_mut().neighbors = tunnels;
        }
        valves.get("AA").unwrap().clone()
    }
}

fn find_optimal_pressure(
    valve: Rc<RefCell<Valve>>,
    time_remaining: u32,
    // use vec instead of set so I can hash it
    mut opened_valves: Vec<String>,
    cache: &mut HashMap<(String, u32, Vec<String>), u32>,
) -> u32 {
    if time_remaining <= 1 {
        return 0;
    }

    let current = valve.borrow();

    let cache_key = (current.name.clone(), time_remaining, opened_valves.clone());
    if let Some(cached_result) = cache.get(&cache_key) {
        return *cached_result;
    }

    let mut paths: Vec<u32> = Vec::new();
    // do not open valve
    for neighbor in current.neighbors.iter() {
        paths.push(find_optimal_pressure(
            neighbor.clone(),
            time_remaining - 1,
            opened_valves.clone(),
            cache,
        ));
    }

    // open valve
    let flow = (time_remaining - 1) * current.flow_rate;
    if flow > 0 && !opened_valves.contains(&current.name) {
        opened_valves.push(current.name.clone());

        for neighbor in current.neighbors.iter() {
            paths.push(
                flow + find_optimal_pressure(
                    neighbor.clone(),
                    time_remaining - 2,
                    opened_valves.clone(),
                    cache,
                ),
            );
        }
    }

    let res = paths.into_iter().max().unwrap();
    cache.insert(cache_key, res);
    res
}

fn problem1(input: &str) -> u32 {
    let start_valve = Valve::from_input(input);
    find_optimal_pressure(start_valve, STARTING_TIME, Vec::new(), &mut HashMap::new())
}

fn problem2(input: &str) -> u32 {
    unimplemented!()
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 1651);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 0);
}
