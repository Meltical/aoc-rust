use std::ops::{Add, Mul, Sub};

pub const DIRECTIONS: [Coord2; 4] =
    [Coord2 { x: 0, y: 1 }, Coord2 { x: 1, y: 0 }, Coord2 { x: 0, y: -1 }, Coord2 { x: -1, y: 0 }];

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord2 {
    pub x: i32,
    pub y: i32,
}

impl Coord2 {
    pub fn new(x: i32, y: i32) -> Coord2 {
        Coord2 { x, y }
    }

    pub fn rotate_cw(&self, n: u8) -> Coord2 {
        let mut new = self.clone();
        for _ in 0..n {
            (new.x, new.y) = (-new.y, new.x);
        }
        new
    }

    /// Returns the mirrored Coord2 of a point (self), by another point
    /// # Examples
    /// For the point (0, 0) and (1, 1), the mirrored coordinate is (2, 2)
    /// ```
    /// use aoc::util::coord::Coord2;
    /// let mirrored = Coord2::new(0, 0).mirrored_by(Coord2::new(1, 1));
    /// assert_eq!(mirrored, Coord2::new(2, 2));
    /// ```
    pub fn mirrored_by(self, other: Coord2) -> Coord2 {
        other * 2 - self
    }

    pub fn distance(self, other: Coord2) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Mul<i32> for Coord2 {
    type Output = Coord2;
    fn mul(self, rhs: i32) -> Self::Output {
        Coord2 { x: self.x * rhs, y: self.y * rhs }
    }
}

// Coord2 [operator] Coord2
impl Mul<Coord2> for Coord2 {
    type Output = Coord2;
    fn mul(self, rhs: Coord2) -> Self::Output {
        Coord2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Add<Coord2> for Coord2 {
    type Output = Coord2;
    fn add(self, rhs: Coord2) -> Self::Output {
        Coord2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub<Coord2> for Coord2 {
    type Output = Coord2;
    fn sub(self, rhs: Coord2) -> Self::Output {
        Coord2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

// &Coord2 [operator] &Coord2
impl Mul<&Coord2> for &Coord2 {
    type Output = Coord2;
    fn mul(self, rhs: &Coord2) -> Self::Output {
        Coord2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Add<&Coord2> for &Coord2 {
    type Output = Coord2;
    fn add(self, rhs: &Coord2) -> Self::Output {
        Coord2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub<&Coord2> for &Coord2 {
    type Output = Coord2;
    fn sub(self, rhs: &Coord2) -> Self::Output {
        Coord2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

// Coord2 [operator] &Coord2
impl Mul<&Coord2> for Coord2 {
    type Output = Coord2;
    fn mul(self, rhs: &Coord2) -> Self::Output {
        Coord2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Add<&Coord2> for Coord2 {
    type Output = Coord2;
    fn add(self, rhs: &Coord2) -> Self::Output {
        Coord2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub<&Coord2> for Coord2 {
    type Output = Coord2;
    fn sub(self, rhs: &Coord2) -> Self::Output {
        Coord2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

// &Coord2 [operator] Coord2
impl Mul<Coord2> for &Coord2 {
    type Output = Coord2;
    fn mul(self, rhs: Coord2) -> Self::Output {
        Coord2 { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Add<Coord2> for &Coord2 {
    type Output = Coord2;
    fn add(self, rhs: Coord2) -> Self::Output {
        Coord2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub<Coord2> for &Coord2 {
    type Output = Coord2;
    fn sub(self, rhs: Coord2) -> Self::Output {
        Coord2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
