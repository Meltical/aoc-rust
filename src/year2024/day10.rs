use crate::util::day::Day;

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
            .filter(|(n, _)| *n != 0)
            .collect::<Vec<(u32, i32)>>();
        let mut end_i = tuples.len() - 1;
        while end_i > 0 {
            let (n, id) = tuples[end_i];
            if id != -1 {
                for j in 0..end_i {
                    let (n1, id1) = tuples[j];
                    if id1 != -1 || n1 < n {
                        continue;
                    }

                    let diff_n = n1 - n;
                    if diff_n == 0 {
                        tuples[j] = (n, id);
                    } else {
                        tuples[j] = (n, id);
                        tuples.insert(j + 1, (diff_n, -1));
                        end_i += 1;
                    }
                    tuples[end_i] = (n, -1);
                    break;
                }
            }
            end_i -= 1;
        }
        let mut i = 0;
        let mut sum = 0;
        for (n, id) in tuples {
            for _ in 0..n {
                if id != -1 {
                    sum += (id as u128) * (i as u128);
                }
                i += 1;
            }
        }
        sum
    }
}
