use std::collections::VecDeque;

use regex::Regex;

use crate::define_solution;

define_solution!(DayThree, part_one, part_two);

fn part_one(lines: Vec<String>) -> String {
    let multiplications: Vec<(i32, i32)> = lines.iter().fold(Vec::new(), |mut acc, next| {
        let pattern = Regex::new(r"mul\((\d+),(\d+)\)").expect("regex failed");
        for (_, [one, two]) in pattern.captures_iter(next).map(|cap| cap.extract()) {
            match (one.parse::<i32>(), two.parse::<i32>()) {
                (Ok(o), Ok(t)) => acc.push((o, t)),
                _ => {}
            }
        }
        acc
    });

    multiplications
        .iter()
        .fold(0, |acc, next| acc + (next.0 * next.1))
        .to_string()
}
fn test_pattern(reg: &Regex, hay: &str, acc: &mut Vec<(i32, i32)>) {
    for (_, [one, two]) in reg.captures_iter(hay).map(|cap| cap.extract()) {
        match (one.parse::<i32>(), two.parse::<i32>()) {
            (Ok(o), Ok(t)) => acc.push((o, t)),
            _ => {}
        }
    }
}

enum Logic {
    Do,
    Dont,
}

fn part_two(lines: Vec<String>) -> String {
    let current_logic = Logic::Do;
    let multiplications: Vec<(i32, i32)> = lines.iter().fold(Vec::new(), |mut acc, next| {
        let pattern = Regex::new(r"mul\((\d+),(\d+)\)").expect("regex failed");
        let mut donts: VecDeque<_> = next.split("don't()").collect();
        if let Some(dont) = donts.pop_front() {
            test_pattern(&pattern, dont, &mut acc);
        };
        for dont in donts {
            if let Some((_, good)) = dont.split_once("do()") {
                test_pattern(&pattern, good, &mut acc);
            };
        }
        acc
    });

    multiplications
        .iter()
        .fold(0, |acc, next| acc + (next.0 * next.1))
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = utils::read_file(&"2024/example3".into()).await;
        assert_eq!(part_one(lines), "161");
    }

    #[tokio::test]
    async fn test_part_two() {
        let lines = vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".into(),
        ];
        assert_eq!(part_two(lines), "48");
    }
}
