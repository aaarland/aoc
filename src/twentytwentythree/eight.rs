use std::collections::HashMap;

use crate::solutions::{Part, Solution, UpdateFn};
pub struct DayEight;

impl Solution for DayEight {
    fn solve(&self, lines: Vec<String>, part: Part, _: Option<UpdateFn>) -> String {
        part_one(lines.clone()).to_string()
    }
}

fn part_one(lines: Vec<String>) -> usize {
    let steps = lines[0].clone();
    let mut network = HashMap::new();
    lines[2..].iter().for_each(|line| {
        let mut split = line.split("=");
        let from = split.next().unwrap().trim();
        let mut to = split
            .next()
            .unwrap()
            .trim()
            .trim_matches(|c| c == '(' || c == ')')
            .split(", ");
        let (left, right) = (to.next().unwrap(), to.next().unwrap());
        network.insert(from, (left, right));
    });
    let mut current = "AAA";
    let mut step_counter = 0;
    while current != "ZZZ" {
        let current_step = steps.chars().nth(step_counter % steps.len()).unwrap();
        let (left, right) = network.get(current).unwrap();
        if current_step == 'L' {
            current = left;
        } else {
            current = right;
        }
        step_counter += 1;
    }
    step_counter
}

fn part_two() {}

#[cfg(test)]
mod tests {
    use crate::utils::read_file;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = read_file(&"2023/example8".into()).await;
        assert_eq!(part_one(lines), 2);
    }

    #[tokio::test]
    async fn test_part_two() {}
}
