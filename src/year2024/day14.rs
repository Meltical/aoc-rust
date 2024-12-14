use crate::util::coord::Coord2;
use crate::util::day::Day;
use crate::util::integer::ParseOps;
use std::collections::HashMap;

#[derive(Debug)]
pub struct State {
    position: Coord2,
    velocity: Coord2,
}

pub struct Day14;
impl Day for Day14 {
    type Input = Vec<State>;
    fn parse(notes: &str) -> Self::Input {
        notes
            .lines()
            .map(|line| {
                let (position_str, velocity_str) = line.split_once(' ').unwrap();
                fn parse(string: &str, pattern: &str) -> Coord2 {
                    let (x, y) = string.trim_start_matches(pattern).split_once(',').unwrap();
                    Coord2::new(x.int(), y.int())
                }
                let position = parse(&position_str, "p=");
                let velocity = parse(&velocity_str, "v=");
                State { position, velocity }
            })
            .collect()
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let final_positions = input.iter().map(|s| move_guard(s)).collect::<Vec<Coord2>>();
        count_quadrant(&final_positions)
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        0
    }
}
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
fn move_guard(state: &State) -> Coord2 {
    let position = state.position;
    let velocity = state.velocity;
    let final_position = position + velocity * 100;
    Coord2::new(
        (final_position.x + (WIDTH * 100)) % WIDTH,
        (final_position.y + (HEIGHT * 100)) % HEIGHT,
    )
}

fn count_quadrant(positions: &Vec<Coord2>) -> u32 {
    let middle_x = WIDTH / 2;
    let middle_y = HEIGHT / 2;
    let mut count = [0; 4];
    positions.iter().for_each(|p| {
        if p.x < middle_x && p.y < middle_y {
            count[0] += 1;
        } else if p.x < middle_x && p.y > middle_y {
            count[1] += 1;
        } else if p.x > middle_x && p.y > middle_y {
            count[2] += 1;
        } else if p.x > middle_x && p.y < middle_y {
            count[3] += 1;
        }
    });
    println!("count: {:?}", count);
    count.iter().fold(1, |acc, cur| acc * cur)
}
