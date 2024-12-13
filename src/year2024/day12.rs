use std::collections::HashSet;
use crate::util::coord::Coord2;
use crate::util::day::Day;
use crate::util::grid::Grid;

pub struct Day12;
impl Day for Day12 {
    type Input = Grid<char>;
    fn parse(notes: &str) -> Self::Input {
        Grid::<char>::new(notes)
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut visited = HashSet::<Coord2>::new();
        let mut i = 0;
        let mut sum = 0;
        while i < input.data.len() {
            let coord = input.coord_at(i as u32);
            let mut tuple = (0, 0);
            if !visited.contains(&coord) {
                get_perimeter_area(input.data[i], coord, &mut tuple, &mut visited, &input);
                sum += tuple.0 * tuple.1;
            }
            i += 1;
        }
        sum
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut visited = HashSet::<Coord2>::new();
        let mut i = 0;
        let mut sum = 0;
        while i < input.data.len() {
            let coord = input.coord_at(i as u32);
            let mut tuple = (0, 0);
            if !visited.contains(&coord) {
                get_edge_area(input.data[i], coord, &mut tuple, &mut visited, &input);
                sum += tuple.0 * tuple.1;
            }
            i += 1;
        }
        sum
    }
}

fn get_perimeter_area(
    value: char,
    coord: Coord2,
    tuple: &mut (u32, u32),
    visited: &mut HashSet<Coord2>,
    grid: &Grid<char>,
) {
    if !visited.insert(coord) {
        return;
    }
    tuple.1 += 1;
    tuple.0 += 4;
    let ortho = [Coord2::new(0, 1), Coord2::new(1, 0), Coord2::new(0, -1), Coord2::new(-1, 0)];
    for dir in ortho {
        let new_coord = coord + dir;
        if grid.at(new_coord).is_some_and(|&v| v == value) {
            tuple.0 -= 1;
            get_perimeter_area(value, new_coord, tuple, visited, grid);
        }
    }
}

fn get_edge_area(
    value: char,
    coord: Coord2,
    tuple: &mut (u32, u32),
    visited: &mut HashSet<Coord2>,
    grid: &Grid<char>,
) {
    if !visited.insert(coord) {
        return;
    }

    tuple.1 += 1;
    let ortho = [Coord2::new(0, 1), Coord2::new(1, 0), Coord2::new(0, -1), Coord2::new(-1, 0)];

    let mut exists = HashSet::<(Coord2, Coord2)>::with_capacity(8);
    for dir in ortho {
        let new_coord = coord + dir;
        if grid.at(new_coord).is_some_and(|&v| v == value) {
            exists.insert((dir, new_coord));
        }
    }

    let directions = exists.iter().map(|&(dir, _)| dir).collect::<Vec<_>>();
    let direction_tuples = directions
        .iter()
        .enumerate()
        .flat_map(|(idx, &dir)| {
            directions.iter().skip(idx + 1).map(|&next_dir| (dir, next_dir)).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for &(a, b) in direction_tuples.iter() {
        let diag_dir = a + b;
        if diag_dir.x != 0 && diag_dir.y != 0 && *grid.at(coord + diag_dir).unwrap_or(&' ') != value
        {
            tuple.0 += 1
        }
    }

    if exists.len() == 0 {
        tuple.0 = 4;
        return;
    }

    if exists.len() == 1 {
        tuple.0 += 2;
    }

    if exists.len() == 2 {
        let diag_dir = direction_tuples[0].0 + direction_tuples[0].1;
        if diag_dir.x != 0 && diag_dir.y != 0 {
            tuple.0 += 1;
        }
    }

    for (_, next_coord) in exists {
        get_edge_area(value, next_coord, tuple, visited, grid);
    }
}
