use crate::util::coord::Coord2;
use crate::util::day::Day;
use crate::util::grid::Grid;
use itertools::Itertools;
use std::collections::HashSet;

pub struct Day15;
impl Day for Day15 {
    type Input = (String, Vec<Coord2>);
    fn parse(notes: &str) -> Self::Input {
        let (grid, inputs) = notes.split_once("\r\n\r\n").unwrap();
        fn parse(c: char) -> Coord2 {
            match c {
                '>' => Coord2::new(1, 0),
                '<' => Coord2::new(-1, 0),
                '^' => Coord2::new(0, -1),
                'v' => Coord2::new(0, 1),
                c => panic!("unexpected character '{}'", c),
            }
        }
        (grid.to_string(), inputs.lines().flat_map(|line| line.chars().map(parse)).collect())
    }

    type Output1 = u32;
    fn part_1((grid_data, inputs): &Self::Input) -> Self::Output1 {
        let grid = Grid::<char>::new(grid_data);
        let mut grid_mutable = grid.clone();
        let mut start =
            grid_mutable.coord_at(grid.data.iter().position(|&c| c == '@').unwrap() as u32);
        for input in inputs {
            step(&mut grid_mutable, &mut start, *input, true);
        }
        count_box(&grid_mutable)
    }

    type Output2 = u32;
    fn part_2((grid_data, inputs): &Self::Input) -> Self::Output2 {
        let grid = parse_grid_part2(grid_data);
        let mut grid_mutable = grid.clone();
        let mut start =
            grid_mutable.coord_at(grid.data.iter().position(|&c| c == '@').unwrap() as u32);
        for input in inputs {
            step(&mut grid_mutable, &mut start, *input, false);
        }
        count_box(&grid_mutable)
    }
}

fn step(grid: &mut Grid<char>, position: &mut Coord2, direction: Coord2, part1: bool) {
    let origin = position.clone();
    let next_start = *position + direction;
    let mut next_position = next_start.clone();
    let mut next_value = grid.at(next_position);
    if next_value.is_none_or(|&c| c == '#') {
        return;
    }

    let grid_immutable = grid.clone();
    if part1 {
        // Count boxes
        let mut box_count = 0f32;
        while next_value.is_some_and(|&c| c == 'O') {
            next_position = next_position + direction;
            next_value = grid.at(next_position);
            box_count += 1f32;
        }
        // If there are boxes but the last value is not an empty space, return early
        if box_count != 0f32 && next_value.is_none_or(|&c| c != '.') {
            return;
        }
        //Move O to the empty space
        for i in 0..box_count as i32 {
            grid.data[grid_immutable.index_at(origin + direction * (i + 2))] = 'O';
        }
    } else {
        // Find all moveable boxes coordinates
        let mut boxes = HashSet::<(Coord2, Coord2)>::new();
        if let Some(box_coord) = get_box_coord(grid, next_position) {
            find_boxes(grid, &mut boxes, box_coord, direction);
        }
        // If any box doesn't have an empty space or another box behind them, return early
        if boxes.iter().any(|&(c1, c2)| {
            grid.at(c1 + direction).is_some_and(|&c| c == '#')
                || grid.at(c2 + direction).is_some_and(|&c| c == '#')
        }) {
            return;
        }
        //Move all boxes, we must sort from furthest to closest otherwise we would override part of a box ('[' or ']') with a '.'
        for current_box in
            boxes.iter().sorted_by(|&&(a, _), &&(b, _)| match (direction.x, direction.y) {
                (0, 1) => b.y.cmp(&a.y),
                (1, 0) => b.x.cmp(&a.x),
                (0, -1) => a.y.cmp(&b.y),
                (-1, 0) => a.x.cmp(&b.x),
                _ => unreachable!(),
            })
        {
            grid.data[grid_immutable.index_at(current_box.0)] = '.';
            grid.data[grid_immutable.index_at(current_box.1)] = '.';
            grid.data[grid_immutable.index_at(current_box.0 + direction)] = '[';
            grid.data[grid_immutable.index_at(current_box.1 + direction)] = ']';
        }
    }

    // Set @ and next_position to next_start
    grid.data[grid_immutable.index_at(next_start)] = '@';
    grid.data[grid_immutable.index_at(origin)] = '.';
    *position = next_start;
}

fn count_box(grid: &Grid<char>) -> u32 {
    grid.data
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'O' || **c == '[')
        .map(|(idx, _)| {
            let coord = grid.coord_at(idx as u32);
            100 * coord.y + coord.x
        })
        .sum::<i32>() as u32
}

fn parse_grid_part2(input: &str) -> Grid<char> {
    let mut result = String::new();
    for line in input.lines() {
        for c in line.chars() {
            match c {
                '.' => result.push_str(".."),
                'O' => result.push_str("[]"),
                '@' => result.push_str("@."),
                '#' => result.push_str("##"),
                c => panic!("unexpected character '{}'", c),
            }
        }
        result.push('\n');
    }
    Grid::<char>::new(result.as_str())
}

fn get_box_coord(grid: &Grid<char>, pos: Coord2) -> Option<(Coord2, Coord2)> {
    match grid.at(pos).unwrap() {
        ']' => Some((Coord2::new(pos.x - 1, pos.y), pos)),
        '[' => Some((pos, Coord2::new(pos.x + 1, pos.y))),
        _ => None,
    }
}

fn find_boxes(
    grid: &Grid<char>,
    boxes: &mut HashSet<(Coord2, Coord2)>,
    current_box: (Coord2, Coord2),
    direction: Coord2,
) {
    if !boxes.insert(current_box) {
        return;
    }
    if let Some(coord) = get_box_coord(grid, current_box.0 + direction) {
        find_boxes(grid, boxes, coord, direction);
    }
    if let Some(coord) = get_box_coord(grid, current_box.1 + direction) {
        find_boxes(grid, boxes, coord, direction);
    }
}
