use crate::define_solution;

define_solution!(DayTwo, part_one, part_two);

struct Range {
    lower: u64,
    upper: u64,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let (lower, upper) = value.split_once('-').unwrap();
        Self {
            lower: lower.parse().unwrap(),
            upper: upper.parse().unwrap(),
        }
    }
}

fn part_one(lines: Vec<String>) -> String {
    let mut total: u64 = 0;
    for line in lines {
        total += line
            .split(',')
            .map(|range_str| Range::from(range_str))
            .fold(0 as u64, |acc, e| {
                acc + (e.lower..=e.upper).fold(0 as u64, |acc, i| {
                    let str_i = i.to_string();
                    let invalid = if str_i.len() % 2 == 0 {
                        let (left, right) = str_i.split_at(str_i.len() / 2);
                        left.chars().zip(right.chars()).all(|(l, r)| l == r)
                    } else {
                        false
                    };
                    if invalid { acc + i } else { acc }
                })
            });
    }
    total.to_string()
}

fn is_invalid(index: u64) -> bool{
    let str_i = index.to_string();
    let max_len = str_i.len() / 2;
    let mut sub_len = 1;
    while sub_len <= max_len {
        let slice = &str_i[..sub_len];
        if (0..str_i.len())
            .step_by(sub_len)
            .all(|i| {
                str_i.len() >= i + sub_len && *slice == str_i[i..i + sub_len]})
        {
            return true;
        }
        sub_len += 1;
    }
    false
}
fn part_two(lines: Vec<String>) -> String {
    let mut total: u64 = 0;
    for line in lines {
        total += line
            .split(',')
            .map(|range_str| Range::from(range_str))
            .fold(0 as u64, |acc, e| {
                acc + (e.lower..=e.upper).fold(0 as u64, |acc, i| {
                    if is_invalid(i) { acc + i } else { acc }
                })
            });
    }
    total.to_string()
}
