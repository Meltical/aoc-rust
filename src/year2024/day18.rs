use crate::util::coord::{Coord2, DIRECTIONS};
use crate::util::day::Day;
use crate::util::integer::ParseOps;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub struct Day18;
impl Day for Day18 {
    type Input = Vec<Coord2>;
    fn parse(notes: &str) -> Self::Input {
        Vec::from_iter(
            notes
                .lines()
                .map(|line| line.split_once(',').unwrap())
                .map(|(x, y)| Coord2::new(x.int::<i32>(), y.int::<i32>())),
        )
    }

    type Output1 = usize;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        solve(&HashSet::from_iter(input.iter().take(1024).cloned()), 71).unwrap()
    }

    type Output2 = String;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        for corruption in 1024..input.len() - 1 {
            if solve(&HashSet::from_iter(input.iter().take(corruption).cloned()), 71).is_none() {
                let mut res = String::new();
                let coord = input[corruption - 1];
                res.push_str([coord.x, coord.y].iter().join(",").as_str());
                return res;
            }
        }
        unreachable!()
    }
}

fn solve(corrupted: &HashSet<Coord2>, size: i32) -> Option<usize> {
    let mut visited = HashSet::<Coord2>::from([Coord2::new(0, 0)]);
    let mut queue = VecDeque::<(Coord2, usize)>::from([(Coord2::new(0, 0), 0)]);
    while let Some((current, cost)) = queue.pop_front() {
        if current == Coord2::new(size - 1, size - 1) {
            return Some(cost);
        }
        for dir in &DIRECTIONS {
            let next = current + dir;
            if visited.contains(&next) || corrupted.contains(&next) {
                continue;
            }
            if next.x < 0 || next.x >= size || next.y < 0 || next.y >= size {
                continue;
            }
            visited.insert(next);
            queue.push_back((next, cost + 1));
        }
    }
    None
}
