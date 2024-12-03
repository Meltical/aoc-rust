use crate::util::integer::ParseOps;
use regex::Regex;

pub fn part1(notes: &str) -> u32 {
    count_mul(notes)
}

const PATTERNS: [&str; 2] = ["do()", "don't()"];
pub fn part2(notes: &str) -> u32 {
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

fn count_mul(notes: &str) -> u32 {
    let mul_re: Regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    notes
        .lines()
        .flat_map(|line| mul_re.captures_iter(line).collect::<Vec<_>>())
        .fold(0, |acc, cap| acc + (&cap[1]).int::<u32>() * (&cap[2]).int::<u32>())
}
