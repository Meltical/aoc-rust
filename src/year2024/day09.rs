use crate::util::day::Day;
use crate::util::integer::ParseOps;
use std::collections::HashMap;

pub struct Day09;
impl Day for Day09 {
    type Input = String;
    fn parse(notes: &str) -> Self::Input {
        notes.to_string()
    }

    //12345 -> 0..111....22222
    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut files = Vec::<i32>::new();
        for (i, c) in input.chars().enumerate() {
            let n = c.to_digit(10).unwrap();
            for _ in 0..n {
                if i % 2 == 0 {
                    files.push((i / 2) as i32);
                } else {
                    files.push(-1);
                }
            }
        }
        let mut sum = 0;
        let mut last_index = files.len() - 1;
        for i in 0..files.len() {
            if files[i] == -1 {
                let (last_idx, last) =
                    files[..last_index + 1].iter().enumerate().rfind(|(_, x)| **x != -1).unwrap();
                last_index = last_idx;
                files[i] = *last;
                files[last_idx] = -1;
            }
            sum += files[i] * (i as i32);
        }
        println!("{:?}", files);
        0
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        0
    }
}
