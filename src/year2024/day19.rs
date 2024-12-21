use crate::util::day::Day;
use std::collections::BTreeSet;

pub struct Day19;
impl Day for Day19 {
    type Input = (Vec<String>, Vec<String>);
    fn parse(notes: &str) -> Self::Input {
        let (patterns, designs) = notes.split_once("\r\n\r\n").unwrap();
        let patterns = patterns.split(", ").map(String::from).collect::<Vec<String>>();
        let designs = designs.lines().map(String::from).collect::<Vec<String>>();
        (patterns, designs)
    }

    type Output1 = usize;
    fn part_1((patterns, designs): &Self::Input) -> Self::Output1 {
        designs.iter().filter(|&design| is_valid(patterns, design)).count()
    }

    type Output2 = usize;
    fn part_2((patterns, designs): &Self::Input) -> Self::Output2 {
        0
    }
}

fn is_valid(patterns: &Vec<String>, design: &String) -> bool {
    let mut queue = BTreeSet::from([0]);
    while let Some(i) = queue.pop_last() {
        if i == design.len() {
            return true;
        }
        for pattern in patterns.iter().filter(|&s| design[i..].starts_with(s)) {
            queue.insert(pattern.len() + i);
        }
    }
    false
}
