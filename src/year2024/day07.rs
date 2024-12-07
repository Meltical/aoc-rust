use crate::util::day::Day;
use crate::util::integer::ParseOps;

pub struct Day07;
impl Day for Day07 {
    type Input = Vec<(u64, Vec<u64>)>;
    fn parse(notes: &str) -> Self::Input {
        notes
            .lines()
            .map(|line| {
                let (result, parts) = line.split_once(':').unwrap();
                (
                    result.int::<u64>(),
                    parts.split_whitespace().map(|v| v.int::<u64>()).collect::<Vec<u64>>(),
                )
            })
            .collect::<Vec<_>>()
    }

    type Output1 = u64;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .filter(|(result, parts)| is_valid(*result, 0, parts, false))
            .fold(0, |acc, (result, _)| acc + result)
    }

    type Output2 = u64;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        input
            .iter()
            .filter(|(result, parts)| is_valid(*result, 0, parts, true))
            .fold(0, |acc, (result, _)| acc + result)
    }
}

fn is_valid(result: u64, sum: u64, parts: &Vec<u64>, part2: bool) -> bool {
    if sum > result {
        return false;
    }
    
    if parts.is_empty() {
        return sum == result;
    }

    let mut next_parts = parts.clone();
    let val = next_parts.remove(0);
    let part1 = is_valid(result, sum + val, &next_parts.clone(), part2)
        || is_valid(result, sum * val, &next_parts.clone(), part2);
    if part2 {
        let concat = (sum.to_string() + val.to_string().as_str()).as_str().int::<u64>();
        return part1 || is_valid(result, concat, &next_parts, part2);
    }
    part1
}
