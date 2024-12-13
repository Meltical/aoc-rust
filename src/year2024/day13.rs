use crate::util::day::Day;

#[derive(Debug)]
pub struct State {
    a: (isize, isize),
    b: (isize, isize),
    p: (isize, isize),
}

pub struct Day13;
impl Day for Day13 {
    type Input = Vec<State>;
    fn parse(notes: &str) -> Self::Input {
        fn parse_button(line: &str) -> (isize, isize) {
            let (x, y) = line.split_once(", ").unwrap();
            (x[x.len() - 2..].parse().unwrap(), y[2..].parse().unwrap())
        }
        fn parse_prize(line: &str) -> (isize, isize) {
            let (x, y) = line.split_once(", ").unwrap();
            (x.trim_start_matches("Prize: X=").parse().unwrap(), y[2..].parse().unwrap())
        }
        notes
            .split("\r\n\r\n")
            .map(|group| {
                let mut lines = group.lines();
                State {
                    a: parse_button(lines.next().unwrap()),
                    b: parse_button(lines.next().unwrap()),
                    p: parse_prize(lines.next().unwrap()),
                }
            })
            .collect()
    }

    type Output1 = isize;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().fold(0isize, |acc, cur| acc + solve(cur, false))
    }

    type Output2 = isize;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.iter().fold(0isize, |acc, cur| acc + solve(cur, true))
    }
}

fn solve(state: &State, part2: bool) -> isize {
    let (a1, a2) = state.a;
    let (b1, b2) = state.b;
    let (mut pa, mut pb) = state.p;
    if part2 {
        pa += 10000000000000;
        pb += 10000000000000;
    }

    let determinant = a1 * b2 - a2 * b1;
    let x = (pa * b2 - pb * b1) as f64 / determinant as f64;
    let y = (pb * a1 - pa * a2) as f64 / determinant as f64;

    if (!part2 && (x > 100f64 || y > 100f64)) || x != x.trunc() || y != y.trunc() {
        0isize
    } else {
        x as isize * 3 + y as isize
    }
}
