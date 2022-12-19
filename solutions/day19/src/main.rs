use std::str::FromStr;

fn main() {
    let input = include_str!("../data/input.txt");
    println!("Problem 1: {}", problem1(input));
    println!("Problem 2: {}", problem2(input));
}

#[derive(Debug, PartialEq, Eq, Default)]
struct Cost {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl Cost {
    fn with_ore(mut self, ore: u16) -> Self {
        self.ore = ore;
        self
    }

    fn with_clay(mut self, clay: u16) -> Self {
        self.clay = clay;
        self
    }

    fn with_obsidian(mut self, obsidian: u16) -> Self {
        self.obsidian = obsidian;
        self
    }

    fn with_geode(mut self, geode: u16) -> Self {
        self.geode = geode;
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u16,
    ore_cost: Cost,
    clay_cost: Cost,
    obsidian_cost: Cost,
    geode_cost: Cost,
}

#[derive(Debug, PartialEq, Eq)]
struct Factory {
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
}

impl Default for Factory {
    fn default() -> Self {
        Self {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct GameState {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    minutes_passed: u16,
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    blueprint: Blueprint,
    factory: Factory,
    state: GameState,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) =
            sscanf::sscanf!(s,"Blueprint {u16}: Each ore robot costs {u16} ore. Each clay robot costs {u16} ore. Each obsidian robot costs {u16} ore and {u16} clay. Each geode robot costs {u16} ore and {u16} obsidian.").unwrap();

        let ore_cost = Cost::default().with_ore(ore_ore);
        let clay_cost = Cost::default().with_ore(clay_ore);
        let obsidian_cost = Cost::default()
            .with_ore(obsidian_ore)
            .with_clay(obsidian_clay);
        let geode_cost = Cost::default()
            .with_ore(geode_ore)
            .with_obsidian(geode_obsidian);

        let blueprint = Blueprint {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
        };

        let factory = Factory::default();
        let state = GameState::default();
        Ok(Self {
            blueprint,
            factory,
            state,
        })
    }
}

fn problem1(input: &str) -> u32 {
    unimplemented!()
}

fn problem2(input: &str) -> u32 {
    unimplemented!()
}

#[test]
fn test_game_from_str() {
    let blueprint_str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
    let actual: Game = blueprint_str.parse().unwrap();
    let expected = Game {
        blueprint: Blueprint {
            id: 1,
            ore_cost: Cost {
                ore: 4,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_cost: Cost {
                ore: 2,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_cost: Cost {
                ore: 3,
                clay: 14,
                obsidian: 0,
                geode: 0,
            },
            geode_cost: Cost {
                ore: 2,
                clay: 0,
                obsidian: 7,
                geode: 0,
            },
        },
        factory: Factory {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        },
        state: GameState {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            minutes_passed: 0,
        },
    };

    assert_eq!(expected, actual);
}

#[test]
fn test_problem1() {
    let input = include_str!("../data/sample.txt");
    let res = problem1(input);
    assert_eq!(res, 33);
}

#[test]
fn test_problem2() {
    let input = include_str!("../data/sample.txt");
    let res = problem2(input);
    assert_eq!(res, 0);
}
