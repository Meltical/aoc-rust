use crate::util::coord::Coord2;
use crate::util::grid::Grid;
use std::collections::HashSet;

pub fn part1(notes: &str) -> u32 {
    count(count_part1, notes, 'X')
}

pub fn part2(notes: &str) -> u32 {
    count(count_part2, notes, 'A')
}

fn count_part1(grid: &Grid<char>, start: Coord2) -> u32 {
    let word = "XMAS".chars().collect::<Vec<char>>();
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0), (-1, -1), (-1, 1), (1, 1), (1, -1)]
        .map(|(x, y)| Coord2::new(x, y));
    let mut sum = 0;
    for dir in directions {
        let mut i = 1;
        while let Some(&c) = grid.at(dir * i + start) {
            if i >= word.len() as i32 || c != word[i as usize] {
                break;
            }
            i += 1;
            if i == word.len() as i32 {
                sum += 1;
            }
        }
    }
    sum
}

fn count_part2(grid: &Grid<char>, start: Coord2) -> u32 {
    let letters = vec!['M', 'S'];
    let diag1 = [Coord2::new(-1, -1), Coord2::new(1, 1)];
    let diag2 = [Coord2::new(-1, 1), Coord2::new(1, -1)];
    let is_valid = |diag: Vec<Coord2>| {
        let set = diag
            .iter()
            .map(|&coord| grid.at(start + coord).unwrap_or(&'.'))
            .collect::<HashSet<_>>();
        set.len() == 2 && letters.iter().all(|&letter| set.contains(&letter))
    };
    (is_valid(Vec::from(diag1)) && is_valid(Vec::from(diag2))) as u32
}

fn count(
    count_fn: fn(grid: &Grid<char>, start: Coord2) -> u32,
    notes: &str,
    start_char: char,
) -> u32 {
    let grid = Grid::new(notes);
    grid.data
        .iter()
        .enumerate()
        .filter(|(_, &c)| c.eq_ignore_ascii_case(&start_char))
        .map(|(idx, _)| count_fn(&grid, grid.coord_at(idx as u32)))
        .sum()
}
