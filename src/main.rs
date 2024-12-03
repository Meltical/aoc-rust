use aoc::*;
use aoc::util::ansi::*;
use std::fs::read_to_string;
use std::iter::empty;

fn main() {
    let solutions = empty().chain(event2024());

    // Pretty print output for each solution
    for Solution { event, day, part1, part2 } in solutions {
        println!("{YELLOW}{event} day {day}{RESET}");
        solve(event, day, 1, part1);
        solve(event, day, 2, part2);
    }
}

fn solve(event: u32, day: u32, part: u32, wrapper: fn(&str) -> String) {
    let path = format!("input/event{event}/aoc_e{event}_d{day:02}_p{part}.txt");

    if let Ok(notes) = read_to_string(&path) {
        println!("    Part {part}: {BOLD}{RED}{}{RESET}", wrapper(&notes));
    } else {
        eprintln!("    Part {part}: {BOLD}{WHITE}{path}{RESET} missing");
    }
}

struct Solution {
    event: u32,
    day: u32,
    part1: fn(&str) -> String,
    part2: fn(&str) -> String,
}

macro_rules! solution {
    ($event:tt, $day:tt) => {{
        use $event::$day::*;

        let event = stringify!($event).strip_prefix("event").unwrap().parse().unwrap();
        let day = stringify!($day).strip_prefix("day").unwrap().parse().unwrap();
        let part1 = |notes: &str| part1(notes).to_string();
        let part2 = |notes: &str| part2(notes).to_string();

        Solution { event, day, part1, part2 }
    }};
}

fn event2024() -> Vec<Solution> {
    vec![solution!(event2024, day01), solution!(event2024, day02), solution!(event2024, day03)]
}
