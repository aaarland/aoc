use std::collections::HashSet;

use crate::solutions::{Part, Solution, UpdateFn};

pub struct DaySix;
impl Solution for DaySix {
    fn solve(&self, lines: Vec<String>, _part: Part, _: Option<UpdateFn>) -> String {
        solution(lines)
    }
}

pub fn solution(lines: Vec<String>) -> String {
    let mut index = 0;
    for _ in lines[0].chars() {
        let mut stack = HashSet::new();
        for n in index..index + 14 {
            if stack.contains(&lines[0].chars().nth(n).unwrap()) {
                break;
            }
            stack.insert(lines[0].chars().nth(n).unwrap());
            // find only unique chars in stack
            if n == index + 13 && stack.len() == 14 {
                return (index + 14).to_string();
            }
        }
        index += 1;
    }
    index.to_string()
}
