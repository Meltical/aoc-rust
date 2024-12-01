use crate::util::integer::ParseOps;
use std::collections::HashMap;

pub fn part1(notes: &str) -> u32 {
    let (mut arr1, mut arr2) = parse(notes);
    arr1.sort_unstable();
    arr2.sort_unstable();
    arr1.iter().zip(arr2.iter()).map(|(&left, &right)| left.abs_diff(right)).sum()
}

pub fn part2(notes: &str) -> u32 {
    let (arr1, arr2) = parse(notes);
    let mut freq = HashMap::new();
    arr2.iter().for_each(|&x| *freq.entry(x).or_insert(0) += 1);
    arr1.iter().map(|&x| x * freq.get(&x).unwrap_or(&0)).sum()
}

fn parse(notes: &str) -> (Vec<u32>, Vec<u32>) {
    notes
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(x, y)| (x.int::<u32>(), y.int::<u32>()))
        .unzip()
}
