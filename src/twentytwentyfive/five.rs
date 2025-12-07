use rayon::prelude::*;
use std::collections::HashSet;

use crate::define_solution;

define_solution!(DayFive, part_one, part_two);

#[derive(Debug)]
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

fn part_two(lines: Vec<String>) -> String {
    let lines_iter = lines.into_iter();
    let mut ranges: Vec<Option<Range>> = lines_iter
        .take_while(|line| !line.is_empty())
        .map(|next| Some(Range::from(next.as_str())))
        .collect();
    let mut updated = false;
    let range_len = ranges.len();
    while !updated {
        updated = false;
        for i in (0..range_len) {
            if let Some(range) = maybe_range {
                for (j, maybe_check_range) in ranges.iter().enumerate() {
                    if let Some(check_range) = maybe_check_range{
                        if is_num_in_range(&range, check_range.lower) || is_num_in_range(&range, check_range.upper){
                            range.lower = check_range.lower.min(range.lower);
                            range.upper = check_range.upper.max(range.upper);
                            updated = true;
                            ranges[j] = None;
                        }

                    }
                }
            }
        }
    }
    "".to_string()
    // we need to combine ranges.
    // let set_of_ranges = ranges.into_par_iter().fold(|| HashSet::new(), |mut acc: HashSet<u64>, range| {
    //     for i in range.lower..=range.upper {
    //         acc.insert(i);
    //     }
    //     acc
    // }).collect_vec_list();
    // println!("Collected all ranges");
    //     set_of_ranges.into_iter().fold(HashSet::new(), |mut acc: HashSet<u64>, next| {
    //     for set in next {
    //         acc.extend(set);
    //     }
    //     acc
    // }).len().to_string()
}
