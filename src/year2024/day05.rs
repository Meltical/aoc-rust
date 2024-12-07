use crate::util::day::Day;
use crate::util::integer::ParseOps;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};

pub struct Day05;
impl Day for Day05 {
    type Input = ([[Ordering; 100]; 100], Vec<Vec<usize>>);
    fn parse(notes: &str) -> Self::Input {
        let (rules, inputs) = notes.split_once("\r\n\r\n").unwrap();
        let rules = rules
            .lines()
            .map(|l| l.split_once('|').unwrap())
            .map(|(a, b)| (a.int::<usize>(), b.int::<usize>()))
            .collect::<Vec<(usize, usize)>>();
        let inputs =
            inputs.lines().map(|l| l.split(',').map(|i| i.int::<usize>()).collect()).collect();
        let mut order = [[Greater; 100]; 100];
        for (a, b) in rules {
            order[a][b] = Less;
        }
        (order, inputs)
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (order, inputs) = input;
        let mut sum = 0;
        for values in inputs {
            if values.is_sorted_by(|&a, &b| order[a][b] == Less) {
                sum += values[values.len() / 2] as u32;
            }
        }
        sum
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (order, inputs) = input;
        let mut sum = 0;
        for values in inputs {
            if !values.is_sorted_by(|&a, &b| order[a][b] == Less) {
                let middle = values.len() / 2;
                let mut values_mut = values.clone();
                values_mut.sort_unstable_by(|&a, &b| order[a][b]);
                sum += values_mut[middle] as u32;
            }
        }
        sum
    }
}
