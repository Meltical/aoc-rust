use crate::util::coord::Coord2;

pub struct Grid<T> {
    pub width: u32,
    pub height: u32,
    pub data: Vec<T>,
}

impl Grid<char> {
    pub fn new(data: &str) -> Grid<char> {
        let lines = data.lines().collect::<Vec<_>>();
        Grid {
            width: lines[0].len() as u32,
            height: lines.len() as u32,
            data: lines.iter().flat_map(|l| l.chars()).collect(),
        }
    }
}

impl<T> Grid<T> {
    pub fn at(&self, point: Coord2) -> Option<&T> {
        let x = point.x;
        let y = point.y;

        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            None
        } else {
            Some(&self.data[(y * self.width as i32 + x) as usize])
        }
    }

    pub fn coord_at(&self, index: u32) -> Coord2 {
        let x = index % self.width;
        let y = index / self.width;
        Coord2 { x: x as i32, y: y as i32 }
    }
}
