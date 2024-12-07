use crate::util::ansi::*;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::time::Instant;

pub trait Day {
    type Input;
    fn parse(notes: &str) -> Self::Input;

    type Output1: Debug;
    fn part_1(input: &Self::Input) -> Self::Output1;

    type Output2: Debug;
    fn part_2(input: &Self::Input) -> Self::Output2;

    fn run_day(year: u16, day: u8) {
        let path = format!("input/year{year}/aoc_y{year}_d{day:02}.txt");

        println!("{YELLOW}{year} day {day}{RESET}");
        if let Ok(notes) = read_to_string(&path) {
            let input = Self::parse(&notes);
            let before1 = Instant::now();
            println!("    Part 1: {BOLD}{RED}{:?}{RESET} - {BOLD}{YELLOW}{:?}{RESET}", Self::part_1(&input), before1.elapsed());
            let before2 = Instant::now();
            println!("    Part 2: {BOLD}{RED}{:?}{RESET} - {BOLD}{YELLOW}{:?}{RESET}", Self::part_2(&input), before2.elapsed());
        } else {
            eprintln!("    Error: {BOLD}{WHITE}{path}{RESET} missing");
        }
    }
}
