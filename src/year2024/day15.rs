use crate::util::coord::Coord2;
use crate::util::day::Day;
use crate::util::grid::Grid;

pub struct Day15;
impl Day for Day15 {
    type Input = (Grid<char>, Vec<Coord2>);
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
        (Grid::<char>::new(grid), inputs.lines().flat_map(|line| line.chars().map(parse)).collect())
    }

    type Output1 = u32;
    fn part_1((grid, inputs): &Self::Input) -> Self::Output1 {
        let mut grid_mutable = grid.clone();
        let mut start =
            grid_mutable.coord_at(grid.data.iter().position(|&c| c == '@').unwrap() as u32);
        for input in inputs {
            step(&mut grid_mutable, &mut start, *input);
        }
        count_box(&grid_mutable)
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        0
    }
}

fn step(grid: &mut Grid<char>, position: &mut Coord2, direction: Coord2) {
    let origin = position.clone();
    let next_start = *position + direction;
    let mut next_position = next_start.clone();
    let mut next_value = grid.at(next_position);
    if next_value.is_none_or(|&c| c == '#') {
        return;
    }

    let mut box_count = 0;
    while next_value.is_some_and(|&c| c == 'O') {
        next_position = next_position + direction;
        next_value = grid.at(next_position);
        box_count += 1;
    }

    if box_count != 0 && next_value.is_none_or(|&c| c != '.') {
        return;
    }

    let grid_immutable = grid.clone();
    grid.data[grid_immutable.index_at(next_start)] = '@';
    grid.data[grid_immutable.index_at(origin)] = '.';
    for i in 0..box_count {
        grid.data[grid_immutable.index_at(origin + direction * (i + 2))] = 'O';
    }
    *position = next_start;
}

fn count_box(grid: &Grid<char>) -> u32 {
    grid.data
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 'O')
        .map(|(idx, _)| {
            let coord = grid.coord_at(idx as u32);
            100 * coord.y + coord.x
        })
        .sum::<i32>() as u32
}
