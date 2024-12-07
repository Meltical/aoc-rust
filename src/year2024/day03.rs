use crate::util::day::Day;
use crate::util::integer::ParseOps;
use regex::Regex;

pub struct Day03;
impl Day for Day03 {
    type Input = String;
    fn parse(notes: &str) -> Self::Input {
        String::from(notes)
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        count_mul(input.as_str())
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let notes = input.as_str();
        let mut enabled = true;
        let mut current_idx = 0;
        let mut sum = 0;
        while let Some(idx) = notes[current_idx..].find(PATTERNS[enabled as usize]) {
            if enabled {
                sum += count_mul(&notes[current_idx..current_idx + idx])
            }
            current_idx += idx + PATTERNS[enabled as usize].len();
            enabled = !enabled;
        }
        if enabled {
            sum += count_mul(&notes[current_idx..]);
        }
        sum
    }
}

const PATTERNS: [&str; 2] = ["do()", "don't()"];
fn count_mul(notes: &str) -> u32 {
    let mul_re: Regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    mul_re
        .captures_iter(notes)
        .fold(0, |acc, cap| acc + (&cap[1]).int::<u32>() * (&cap[2]).int::<u32>())
}
