use std::cmp::Ordering;

use crate::define_solution;

define_solution!(DayFive, part_one, part_two);

#[derive(Debug, PartialEq, PartialOrd)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Eq for Range {}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.lower < other.lower || self.upper < other.upper {
            Ordering::Less
        } else if self.upper == other.upper && self.lower == other.lower {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
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

fn is_num_in_range(range: &Range, num: u64) -> bool {
    range.lower <= num && range.upper >= num
}

fn part_one(lines: Vec<String>) -> String {
    let lines_iter = lines.into_iter();
    let ranges: Vec<Range> = lines_iter
        .clone()
        .take_while(|line| !line.is_empty())
        .map(|next| Range::from(next.as_str()))
        .collect();
    lines_iter
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .fold(0, |acc, next| {
            let parsed: u64 = next.parse().unwrap();
            if ranges.iter().any(|range| is_num_in_range(range, parsed)) {
                acc + 1
            } else {
                acc
            }
        })
        .to_string()
}

fn update_range(ranges: &mut Vec<Option<Range>>, current_index: usize) {
    let (left, right) = ranges.split_at_mut(current_index + 1);
    let Some(ref mut range) = left[current_index] else {
        return;
    };
    for maybe_check_range in right {
        let Some(check_range) = maybe_check_range else {
            continue;
        };
        if is_num_in_range(&range, check_range.lower) || is_num_in_range(&range, check_range.upper)
        {
            range.lower = check_range.lower.min(range.lower);
            range.upper = check_range.upper.max(range.upper);
            maybe_check_range.take();
        }
    }
}

fn part_two(lines: Vec<String>) -> String {
    let lines_iter = lines.into_iter();
    let mut ranges: Vec<Option<Range>> = lines_iter
        .take_while(|line| !line.is_empty())
        .map(|next| Some(Range::from(next.as_str())))
        .collect();
    ranges.sort();
    let range_len = ranges.len();
    for _ in 0..range_len {
        for i in 0..range_len {
            update_range(&mut ranges, i);
        }
    }
    let mut all_ranges = ranges.iter().fold(Vec::new(), |mut acc, next| {
        if let Some(range) = next {
            acc.push(range.lower);
            acc.push(range.upper);
        }
        acc
    });
    all_ranges.sort();
    dbg!(&ranges);
    ranges
        .into_iter()
        .fold(0, |acc, next| {
            let Some(range) = next else { return acc };
            acc + (range.upper + 1 - range.lower)
        })
        .to_string()
}
