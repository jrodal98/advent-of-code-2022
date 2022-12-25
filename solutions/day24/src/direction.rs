#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stay,
}

impl Direction {
    pub fn iterator(prefer_down_right: bool) -> impl Iterator<Item = Self> {
        if prefer_down_right {
            [Self::Down, Self::Right, Self::Stay, Self::Up, Self::Left]
                .iter()
                .copied()
        } else {
            [Self::Up, Self::Left, Self::Stay, Self::Down, Self::Right]
                .iter()
                .copied()
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => Self::Stay,
        }
    }
}
