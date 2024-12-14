use crate::util::coord::Coord2;
use crate::util::day::Day;
use crate::util::grid::Grid;
use crate::util::integer::ParseOps;

#[derive(Debug, Clone)]
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
        let final_positions = input.iter().map(|s| move_guard(s, 100)).collect::<Vec<Coord2>>();
        count_quadrant(&final_positions)
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut positions = input.to_vec();
        let mut i = 1;
        loop {
            positions = positions
                .iter()
                .map(|s| State { position: move_guard(s, 1), velocity: s.velocity })
                .collect::<Vec<State>>();
            let variance_x = variance(&positions);
            if variance_x < 800f32 {
                // to_grid(positions).log();
                return i as u32;
            }
            i += 1;
        }
    }
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
fn move_guard(state: &State, n: i32) -> Coord2 {
    let position = state.position;
    let velocity = state.velocity;
    let final_position = position + velocity * n;
    Coord2::new(
        (final_position.x + (WIDTH * n)) % WIDTH,
        (final_position.y + (HEIGHT * n)) % HEIGHT,
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
    count.iter().fold(1, |acc, cur| acc * cur)
}

fn variance(positions: &Vec<State>) -> f32 {
    let len = positions.len();
    let mean =
        positions.iter().fold(0, |acc, cur| acc + cur.position.x + cur.position.y) / len as i32;
    let differences =
        positions.iter().map(|s| (s.position.x + s.position.y - mean).pow(2)).collect::<Vec<i32>>();
    differences.iter().sum::<i32>() as f32 / len as f32
}

fn to_grid(positions: Vec<State>) -> Grid<char> {
    let data = [0; HEIGHT as usize]
        .map(|_| [0; WIDTH as usize].map(|_| '.').iter().collect::<String>())
        .join("\n");
    let mut grid = Grid::<char>::new(data.as_str());
    let grid_copy = grid.clone();
    positions.iter().for_each(|p| grid.data[grid_copy.index_at(p.position)] = 'X');
    grid
}
