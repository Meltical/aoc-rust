use crate::util::day::Day;
use itertools::Itertools;
use std::collections::HashMap;
use std::mem;

pub struct Day11;
impl Day for Day11 {
    type Input = HashMap<usize, usize>;
    fn parse(notes: &str) -> Self::Input {
        notes.split(' ').map(|n| n.parse().unwrap()).counts()
    }

    type Output1 = usize;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        solve(input.clone(), 25)
    }

    type Output2 = usize;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        solve(input.clone(), 75)
    }
}

//https://github.com/AhmedYassineMaalej/AoC-2024/blob/master/src/problems/day11.rs
fn solve(mut stones: HashMap<usize, usize>, n: u32) -> usize {
    let mut new_stones: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        for (stone, count) in stones.drain() {
            if stone == 0 {
                *new_stones.entry(1).or_default() += count;
            } else {
                let digit_count = stone.ilog10() + 1;
                if digit_count % 2 == 0 {
                    let power = 10usize.pow(digit_count / 2);
                    let (left, right) = (stone / power, stone % power);
                    *new_stones.entry(left).or_default() += count;
                    *new_stones.entry(right).or_default() += count;
                } else {
                    *new_stones.entry(stone * 2024).or_default() += count;
                }
            }
        }
        mem::swap(&mut stones, &mut new_stones);
    }
    stones.values().sum()
}
