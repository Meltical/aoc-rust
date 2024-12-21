use crate::util::coord::{Coord2, DIRECTIONS};
use crate::util::day::Day;
use crate::util::grid::Grid;
use std::collections::{BinaryHeap, HashMap};

pub struct Day16;
impl Day for Day16 {
    type Input = Grid<char>;
    fn parse(notes: &str) -> Self::Input {
        Grid::<char>::new(notes)
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let grid = input.clone();
        let start = grid.coord_at(input.data.iter().position(|&c| c == 'S').unwrap() as u32);
        bfs(&grid, start)
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        0
    }
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct State {
    cost: u32,
    position: Coord2,
    direction: Coord2,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(grid: &Grid<char>, start: Coord2) -> u32 {
    let mut costs = HashMap::<(Coord2, Coord2), u32>::new();
    let start = State { cost: 0, position: start, direction: Coord2::new(1, 0) };
    let mut queue = BinaryHeap::from([start]);

    while let Some(next) = queue.pop() {
        if grid.at(next.position) == Some(&'E') {
            return next.cost;
        }

        for &dir in &DIRECTIONS {
            let next_pos = next.position + dir;
            if grid.at(next_pos) == Some(&'#') {
                continue;
            }
            let mut next_cost = &next.cost + 1;
            if next.direction.x != dir.x && next.direction.y != dir.y {
                next_cost += 1000;
            }

            let cost_key = (next_pos, dir);
            if costs.get(&cost_key).map_or(true, |&existing_cost| next_cost < existing_cost) {
                let next_state = State { cost: next_cost, position: next_pos, direction: dir };

                costs.insert(cost_key, next_cost);
                queue.push(next_state);
            }
        }
    }
    unreachable!("No path found");
}
