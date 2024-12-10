use crate::util::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day09;
impl Day for Day09 {
    type Input = String;
    fn parse(notes: &str) -> Self::Input {
        notes.to_string()
    }

    type Output1 = u64;
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
                if last_idx < i {
                    break;
                }
                last_index = last_idx;
                files[i] = *last;
                files[last_idx] = -1;
            }
            sum += (files[i] as u64) * (i as u64);
        }
        sum
    }

    type Output2 = u128;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut tuples = input
            .chars()
            .enumerate()
            .map(|(idx, c)| {
                return if idx % 2 == 0 {
                    (c.to_digit(10).unwrap(), (idx / 2) as i32)
                } else {
                    (c.to_digit(10).unwrap(), -1)
                };
            })
            .collect::<Vec<(u32, i32)>>();
        println!("{:?}", tuples);
        for mut i in (0..tuples.len()).rev() {
            let (x, n) = tuples[i];
            for j in 0..i {
                let (x1, n1) = tuples[j];
                if n1 != -1 || x1 < x {
                    continue;
                }
                let z = x1 - x;
                if z == 0 {
                    tuples[j] = (x1, n);
                    tuples[i] = (x, -1);
                    i += 1;
                } else {
                    tuples[j] = (x, n);
                    let (x2, n2) = tuples[j + 1];
                    if n2 == -1 {
                        tuples.remove(j + 2);
                        tuples[j + 1] = (z + x2, -1);
                    } else {
                        tuples.insert(j + 1, (z, -1));
                    }
                }
                tuples[i] = (x, -1);
                break;
            }
        }
        println!("{:?}", tuples);
        0
    }
}
// 00992111777.44.333....5555.6666.....8888..