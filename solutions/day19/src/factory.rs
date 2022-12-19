use std::ops::{AddAssign, SubAssign};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, Default)]
pub struct FactoryOutput {
    pub ore: u16,
    pub clay: u16,
    pub obsidian: u16,
    pub geode: u16,
}

impl AddAssign for FactoryOutput {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl SubAssign for FactoryOutput {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl FactoryOutput {
    pub fn with_ore(mut self, ore: u16) -> Self {
        self.ore = ore;
        self
    }

    pub fn with_clay(mut self, clay: u16) -> Self {
        self.clay = clay;
        self
    }

    pub fn with_obsidian(mut self, obsidian: u16) -> Self {
        self.obsidian = obsidian;
        self
    }

    pub fn can_afford(&self, other: &Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geode >= other.geode
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Factory {
    pub ore_robots: u16,
    pub clay_robots: u16,
    pub obsidian_robots: u16,
    pub geode_robots: u16,
}

impl Factory {
    pub fn produce(&self) -> FactoryOutput {
        FactoryOutput {
            ore: self.ore_robots,
            clay: self.clay_robots,
            obsidian: self.obsidian_robots,
            geode: self.geode_robots,
        }
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blueprint {
    pub id: u16,
    pub ore_cost: FactoryOutput,
    pub clay_cost: FactoryOutput,
    pub obsidian_cost: FactoryOutput,
    pub geode_cost: FactoryOutput,
}

impl Blueprint {
    pub fn max_ore(&self) -> u16 {
        self.ore_cost.ore.max(
            self.clay_cost
                .ore
                .max(self.obsidian_cost.ore.max(self.geode_cost.ore)),
        )
    }
    pub fn max_clay(&self) -> u16 {
        self.obsidian_cost.clay
    }

    pub fn max_obsidian(&self) -> u16 {
        self.geode_cost.obsidian
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_afford() {
        let output1 = FactoryOutput::default().with_ore(2).with_obsidian(7);
        let output2 = FactoryOutput::default().with_ore(2).with_clay(1);
        assert_eq!(false, output2.can_afford(&output1));
    }
}
