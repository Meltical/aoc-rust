use crate::util::day::Day;
use crate::util::integer::ParseOps;
use std::collections::HashMap;

pub struct Day01;
impl Day for Day01 {
    type Input = (Vec<u32>, Vec<u32>);
    fn parse(notes: &str) -> Self::Input {
        notes
            .lines()
            .map(|line| line.split_once("   ").unwrap())
            .map(|(x, y)| (x.int::<u32>(), y.int::<u32>()))
            .unzip()
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (mut arr1, mut arr2) = input.clone();
        arr1.sort_unstable();
        arr2.sort_unstable();
        arr1.iter().zip(arr2.iter()).map(|(&left, &right)| left.abs_diff(right)).sum()
    }

    type Output2 = u32;
    fn part_2((arr1, arr2): &Self::Input) -> Self::Output2 {
        let mut freq = HashMap::new();
        arr2.iter().for_each(|&x| *freq.entry(x).or_insert(0) += 1);
        arr1.iter().map(|&x| x * freq.get(&x).unwrap_or(&0)).sum()
    }
}
