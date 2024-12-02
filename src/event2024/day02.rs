use crate::util::integer::ParseOps;

pub fn part1(notes: &str) -> u32 {
    parse(notes).iter().filter(is_valid).count() as u32
}

pub fn part2(notes: &str) -> u32 {
    let mut sum = 0;
    for report in parse(notes) {
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

fn parse(notes: &str) -> Vec<Vec<u32>> {
    notes.lines().map(|line| line.split(' ').map(|x| x.int::<u32>()).collect()).collect()
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
