use std::str::FromStr;

use crate::factory::{Blueprint, Factory, FactoryOutput};

#[derive(Debug, Hash, PartialEq, Eq, Default)]
pub struct GameState {
    factory: Factory,
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    minutes_passed: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    blueprint: Blueprint,
    state: GameState,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian) =
            sscanf::sscanf!(s,"Blueprint {u16}: Each ore robot costs {u16} ore. Each clay robot costs {u16} ore. Each obsidian robot costs {u16} ore and {u16} clay. Each geode robot costs {u16} ore and {u16} obsidian.").unwrap();

        let ore_cost = FactoryOutput::default().with_ore(ore_ore);
        let clay_cost = FactoryOutput::default().with_ore(clay_ore);
        let obsidian_cost = FactoryOutput::default()
            .with_ore(obsidian_ore)
            .with_clay(obsidian_clay);
        let geode_cost = FactoryOutput::default()
            .with_ore(geode_ore)
            .with_obsidian(geode_obsidian);

        let blueprint = Blueprint {
            id,
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
        };

        let state = GameState::default();
        Ok(Self { blueprint, state })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_str() {
        let blueprint_str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let actual: Game = blueprint_str.parse().unwrap();
        let expected = Game {
            blueprint: Blueprint {
                id: 1,
                ore_cost: FactoryOutput {
                    ore: 4,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                clay_cost: FactoryOutput {
                    ore: 2,
                    clay: 0,
                    obsidian: 0,
                    geode: 0,
                },
                obsidian_cost: FactoryOutput {
                    ore: 3,
                    clay: 14,
                    obsidian: 0,
                    geode: 0,
                },
                geode_cost: FactoryOutput {
                    ore: 2,
                    clay: 0,
                    obsidian: 7,
                    geode: 0,
                },
            },
            state: GameState {
                factory: Factory {
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
                minutes_passed: 0,
            },
        };

        assert_eq!(expected, actual);
    }
}
