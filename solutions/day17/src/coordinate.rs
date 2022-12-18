#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn translate_new(&self, dx: isize, dy: isize) -> Self {
        Self::new(
            (self.x as isize + dx) as usize,
            (self.y as isize + dy) as usize,
        )
    }
}
