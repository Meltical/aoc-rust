use crate::util::day::Day;
use crate::util::integer::ParseOps;

pub struct Day02;
impl Day for Day02 {
    type Input = Vec<Vec<u32>>;
    fn parse(notes: &str) -> Self::Input {
        notes.lines().map(|line| line.split(' ').map(|x| x.int::<u32>()).collect()).collect()
    }

    type Output1 = u32;
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().filter(is_valid).count() as u32
    }

    type Output2 = u32;
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut sum = 0;
        for report in input {
            if is_valid(&&report) {
                sum += 1;
                continue;
            }
            for i in 0..report.len() {
                let mut changed_report = report.clone();
                changed_report.remove(i);
                if is_valid(&&changed_report) {
                    sum += 1;
                    break;
                }
            }
        }
        sum
    }
}

fn is_valid(report: &&Vec<u32>) -> bool {
    let is_less = report[0] < report[1];
    for i in 1..report.len() {
        let prev = report[i - 1];
        let val = report[i];
        let diff = val.abs_diff(prev);
        if (prev < val) != is_less || diff == 0 || diff > 3 {
            return false;
        }
    }
    true
}
