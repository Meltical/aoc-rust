use crate::util::coord::Coord2;
use crate::util::day::Day;
use crate::util::grid::Grid;
use std::collections::{HashMap, HashSet};

pub struct Day08;
impl Day for Day08 {
    type Input = (Grid<char>, HashMap<char, HashSet<Coord2>>);
    fn parse(notes: &str) -> Self::Input {
        let grid = Grid::new(notes);
        let mut coords = HashMap::<char, HashSet<Coord2>>::new();
        for (idx, &c) in grid.data.iter().enumerate().filter(|(_, &c)| c != '.') {
            coords.entry(c).or_insert_with(HashSet::new).insert(grid.coord_at(idx as u32));
        }
        (grid, coords)
    }

    type Output1 = u32;
    fn part_1((grid, coords): &Self::Input) -> Self::Output1 {
        count_antinode(&coords, |(a, b)| get_antinodes(&a, &b, grid, false))
    }

    type Output2 = u32;
    fn part_2((grid, coords): &Self::Input) -> Self::Output2 {
        count_antinode(coords, |(a, b)| get_antinodes(&a, &b, grid, true))
    }
}

fn get_antinodes(start: &Coord2, end: &Coord2, grid: &Grid<char>, part_2: bool) -> HashSet<Coord2> {
    let mut queue = vec![(*end, start.mirrored_by(*end)), (*start, end.mirrored_by(*start))];
    let mut antinodes = HashSet::<Coord2>::new();
    if part_2 {
        antinodes.insert(*start);
        antinodes.insert(*end);
    }
    while let Some((start, end)) = queue.pop() {
        let coord = grid.at(end);
        if coord.is_some() {
            antinodes.insert(end); //Add new antinode
        }

        if part_2 {
            if coord.is_some() {
                queue.push((end, start.mirrored_by(end)));
            }
        }
    }
    antinodes
}

fn count_antinode<F>(coords: &HashMap<char, HashSet<Coord2>>, getter: F) -> u32
where
    F: Fn((Coord2, Coord2)) -> HashSet<Coord2>,
{
    let mut antinodes = HashSet::<Coord2>::new();
    for values in coords.values() {
        for (idx, &a) in values.iter().enumerate() {
            for &b in values.iter().skip(idx + 1) {
                for val in getter((a, b)) {
                    antinodes.insert(val);
                }
            }
        }
    }
    antinodes.len() as u32
}
