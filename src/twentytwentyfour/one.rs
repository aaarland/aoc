use std::collections::HashMap;

use crate::solutions::Solution;
pub struct DayOne;

impl Solution for DayOne {
    fn solve(&self, lines: Vec<String>) -> () {
        let total = part_two(lines);
        println!("Total: {}", total);
    }
}

fn parse_lines(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let line_length = lines.len();
    lines.iter().fold(
        (
            Vec::with_capacity(line_length),
            Vec::with_capacity(line_length),
        ),
        |(mut first, mut last), next| {
            let mut iter = next.split_whitespace();
            let one = iter
                .next()
                .expect("Malformed input")
                .parse::<i32>()
                .expect("failed to parse");
            let two = iter
                .next()
                .expect("Malformed input")
                .parse::<i32>()
                .expect("failed to parse");
            first.push(one);
            last.push(two);
            (first, last)
        },
    )
}

fn part_one(lines: Vec<String>) -> i32 {
    let (mut first, mut last) = parse_lines(lines);

    first.sort();
    last.sort();
    first
        .iter()
        .zip(last.iter())
        .fold(0, |current, (one, two)| {
            let distance = (one - two).abs();
            current + distance
        })
}

fn part_two(lines: Vec<String>) -> i32 {
    let (first, last) = parse_lines(lines);
    let mut map = first.iter().fold(HashMap::new(), |mut current, next| {
        current.insert(*next, 0);
        current
    });
    for num in &last {
        map.entry(*num).and_modify(|e| *e += 1);
    }
    first.iter().fold(0, |current, next| {
        current + (next * map.get(next).unwrap_or(&0))
    })
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = utils::read_file(&"2024/example1".into());
        assert_eq!(part_one(lines), 11)
    }

    #[test]
    fn test_part_two() {
        let lines = utils::read_file(&"2024/example1".into());
        assert_eq!(part_two(lines), 31)
    }
}
