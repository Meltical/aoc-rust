use crate::util::coord::Coord2;
use crate::util::day::Day;
use crate::util::grid::Grid;
use std::collections::HashSet;

pub struct Day10;
impl Day for Day10 {
    type Input = Grid<u32>;
    fn parse(notes: &str) -> Self::Input {
        Grid::<u32>::new(notes)
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut sum = 0;
        input.data.iter().enumerate().filter(|(_, v)| **v == 0).for_each(|(i, _)| {
            step(input, input.coord_at(i as u32), &mut sum, &mut HashSet::<Coord2>::new(), false);
        });
        sum
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut sum = 0;
        input.data.iter().enumerate().filter(|(_, v)| **v == 0).for_each(|(i, _)| {
            step(input, input.coord_at(i as u32), &mut sum, &mut HashSet::<Coord2>::new(), true);
        });
        sum
    }
}

fn step(
    grid: &Grid<u32>,
    position: Coord2,
    sum: &mut u32,
    trailends: &mut HashSet<Coord2>,
    part2: bool,
) {
    let current = *grid.at(position).unwrap();
    if current == 9 && (trailends.insert(position) || part2) {
        *sum += 1;
    }
    let ortho = [Coord2::new(0, 1), Coord2::new(1, 0), Coord2::new(0, -1), Coord2::new(-1, 0)];
    for dir in ortho {
        let next_pos = position + dir;
        if let Some(next) = grid.at(next_pos) {
            let expected = current + 1;
            if *next == expected {
                step(grid, next_pos, sum, trailends, part2);
            }
        }
    }
}
