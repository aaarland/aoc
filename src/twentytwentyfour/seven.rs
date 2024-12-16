use core::panic;

use crate::define_solution;

define_solution!(DaySeven, part_one, part_two);

fn verify_possible(result: i64, testers: Vec<i64>) -> bool {
    for index in 0..i64::pow(2, testers.len() as u32) {
        let res = testers
            .iter()
            .enumerate()
            .fold(0, |current, (inner_index, next)| {
                let shifted_index = (index >> inner_index) & 1;
                match shifted_index {
                    0 => current + next,
                    1 => current * next,
                    _ => panic!("Value was outside of byte"),
                }
            });
        if res == result {
            return true;
        };
    }
    false
}
fn part_one(lines: Vec<String>) -> String {
    lines.iter().fold(0, |current, next| {
        let Some(res) = next.split_once(":") else {
            return current;
        };
        let result: i64 = res.0.parse().unwrap();
        let testers: Vec<i64> = res
            .1
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if verify_possible(result, testers) {
            current + result
        } else {
            current
        }
    }).to_string()
}

fn part_two(lines: Vec<String>) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = utils::read_db(7, 2024, &"example".into()).await;
        assert_eq!("3749", part_one(lines));
    }

    #[tokio::test]
    async fn test_part_two() {}
}
