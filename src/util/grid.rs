use std::ops::{Index, IndexMut};
use crate::util::coord::Coord2;

#[derive(Clone)]
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

impl Grid<u32> {
    pub fn new(data: &str) -> Grid<u32> {
        let lines = data.lines().collect::<Vec<_>>();
        Grid {
            width: lines[0].len() as u32,
            height: lines.len() as u32,
            data: lines.iter().flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap())).collect(),
        }
    }
}

impl<T: std::fmt::Display + PartialEq> Grid<T> {
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

    pub fn index_at(&self, coord: Coord2) -> usize {
        let x = coord.x;
        let y = coord.y;
        (y * self.width as i32 + x) as usize
    }

    pub fn log(&self) {
        for chunk in self.data.chunks(self.width as usize) {
            for c in chunk {
                print!("{c}");
            }
            println!();
        }
    }

    pub fn find(&self, value: T) -> Option<Coord2> {
        if let Some(position) = self.data.iter().position(|v| *v == value) {
            return Some(self.coord_at(position as u32));
        }
        None
    }

    pub fn new_with<U: Copy>(&self, value: U) -> Grid<U> {
        Grid {
            width: self.width,
            height: self.height,
            data: vec![value; (self.width * self.height) as usize]
        }
    }
}
