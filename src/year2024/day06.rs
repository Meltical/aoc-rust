use crate::util::coord::Coord2;
use crate::util::day::Day;
use crate::util::grid::Grid;
use std::collections::HashSet;

pub struct Day06;
impl Day for Day06 {
    type Input = (u32, u32);
    fn parse(notes: &str) -> Self::Input {
        let initial_grid = Grid::<char>::new(notes);
        let start_idx = initial_grid.data.iter().position(|v| *v == '^').unwrap();
        let start = State {
            position: initial_grid.coord_at(start_idx as u32),
            direction: Coord2 { x: 0, y: -1 },
        };
        let run = |grid: &Grid<char>| {
            let mut visited = HashSet::<State>::new();
            let mut queue = Vec::<State>::from([start]);
            while let Some(state) = queue.pop() {
                let pos = state.position;
                let dir = state.direction;
                if !visited.insert(state) {
                    return None;
                }

                let directions = [dir, dir.rotate_cw(1), dir.rotate_cw(2), dir.rotate_cw(3)];
                let next_dir = *directions
                    .iter()
                    .find(|&&dir| grid.at(pos + dir).is_none_or(|v| v != &'#'))
                    .unwrap();
                let next_pos = pos + next_dir;
                if let Some(_) = grid.at(next_pos) {
                    let next = State { position: next_pos, direction: next_dir };
                    queue.push(next);
                }
            }
            Some(visited)
        };

        let path =
            HashSet::<Coord2>::from_iter(run(&initial_grid).unwrap().iter().map(|s| s.position));
        let part1 = path.len() as u32;
        let mut part2 = 0;
        let mut new_grid;
        for pos in path {
            new_grid = initial_grid.clone();
            new_grid.data[initial_grid.index_at(pos)] = '#';
            if run(&new_grid) == None {
                part2 += 1;
            }
        }

        (part1, part2)
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.0
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.1
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct State {
    position: Coord2,
    direction: Coord2,
}
