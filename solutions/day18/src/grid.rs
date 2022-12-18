use std::collections::HashSet;

use crate::coordinate::Coordinate;

pub struct Grid {
    coordinates: HashSet<Coordinate>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let coordinates: HashSet<Coordinate> =
            input.lines().map(|line| line.parse().unwrap()).collect();
        Self { coordinates }
    }

    pub fn area_including_air_pockets(&self) -> usize {
        Self::area(&self.coordinates)
    }

    pub fn area_excluding_air_pockets(&self) -> usize {
        let including_pockets = self.area_including_air_pockets();
        let air_pocket_penalty = Self::area(&self.get_air_pockets());

        including_pockets - air_pocket_penalty
    }

    fn area(coordinates: &HashSet<Coordinate>) -> usize {
        coordinates
            .iter()
            .flat_map(|c| c.adjacent_coordinates())
            .filter(|c| !coordinates.contains(c))
            .count()
    }

    fn get_air_pockets(&self) -> HashSet<Coordinate> {
        let min_x = self.coordinates.iter().map(|c| c.x).min().unwrap_or(0);
        let max_x = self.coordinates.iter().map(|c| c.x).max().unwrap_or(0);
        let min_y = self.coordinates.iter().map(|c| c.y).min().unwrap_or(0);
        let max_y = self.coordinates.iter().map(|c| c.y).max().unwrap_or(0);
        let min_z = self.coordinates.iter().map(|c| c.z).min().unwrap_or(0);
        let max_z = self.coordinates.iter().map(|c| c.z).max().unwrap_or(0);

        let mut pockets: HashSet<Coordinate> = (min_x - 1..=max_x + 1)
            .flat_map(|x| (min_y - 1..=max_y + 1).map(move |y| (x, y)))
            .flat_map(|(x, y)| (min_z - 1..=max_z + 1).map(move |z| Coordinate { x, y, z }))
            .filter(|coord| !self.coordinates.contains(coord))
            .collect();

        let mut cubes_to_explore = vec![Coordinate {
            x: min_x - 1,
            y: min_y - 1,
            z: min_z - 1,
        }];

        // Simulate "being the water"
        while let Some(cube) = cubes_to_explore.pop() {
            // if the water is able to reach the cube, then it isn't an air pocket
            if pockets.remove(&cube) {
                // continue exploring along the space
                cubes_to_explore.extend(cube.adjacent_coordinates());
            }
        }

        pockets
    }
}
