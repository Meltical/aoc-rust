use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Coord2 {
    pub x: i32,
    pub y: i32,
}

impl Coord2 {
    pub fn new(x: i32, y: i32) -> Coord2 {
        Coord2 { x, y }
    }
}

impl Mul<Coord2> for Coord2 {
    type Output = Coord2;
    fn mul(self, rhs: Coord2) -> Self::Output {
        Coord2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Mul<i32> for Coord2 {
    type Output = Coord2;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Add<Coord2> for Coord2 {
    type Output = Coord2;
    fn add(self, rhs: Coord2) -> Self::Output {
        Coord2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
