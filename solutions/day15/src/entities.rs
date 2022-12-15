use std::str::FromStr;
pub const MAX_XY: isize = 4000000;

#[derive(Debug)]
pub struct Sensor {
    pub position: Coordinate,
    pub beacon: Coordinate,
    pub distance_to_beacon: usize,
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // skip sensor,at
        let mut split_str = s.split_whitespace().skip(2);
        let x_str = split_str.next().unwrap();
        // exclude x= and ,
        let x = x_str[2..x_str.len() - 1].parse::<isize>().unwrap();

        let y_str = split_str.next().unwrap();
        // exclude y= and :
        let y = y_str[2..y_str.len() - 1].parse::<isize>().unwrap();

        let position = Coordinate { x, y };

        // skip closest,beacon,is,at
        let mut split_str = split_str.skip(4);

        let x_str = split_str.next().unwrap();
        // exclude x= and ,
        let x = x_str[2..x_str.len() - 1].parse::<isize>().unwrap();

        let y_str = split_str.next().unwrap();
        // exclude y=
        let y = y_str[2..].parse::<isize>().unwrap();

        let beacon = Coordinate { x, y };

        let distance_to_beacon = position.manhattan_distance(&beacon);

        Ok(Self {
            position,
            beacon,
            distance_to_beacon,
        })
    }
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Coordinate {
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn tuning_distance(&self) -> usize {
        (self.x * MAX_XY + self.y) as usize
    }
}
