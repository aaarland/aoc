use std::collections::HashMap;

use crate::define_solution;

define_solution!(DaySix, part_one, part_two);

enum Ops {
    Plus,
    Multiply,
}
use Ops::*;
impl Ops {
    fn calculate_next(&self, entry: u64, parsed: u64) -> u64 {
        match self {
            Plus => entry + parsed,
            Multiply => entry * parsed,
        }
    }
}

impl From<&str> for Ops {
    fn from(value: &str) -> Self {
        match value {
            "*" => Self::Multiply,
            "+" => Self::Plus,
            _ => panic!("not a valid op"),
        }
    }
}

fn part_one(lines: Vec<String>) -> String {
    let ops: Vec<_> = lines
        .iter()
        .rev()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|next| Ops::from(next))
        .collect();
    lines[..lines.len() - 1]
        .iter()
        .map(|next| next.split_whitespace())
        .fold(HashMap::new(), |mut acc: HashMap<usize, u64>, next| {
            for (i, num) in next.enumerate() {
                let parsed = num.parse().unwrap();
                acc.entry(i)
                    .and_modify(|entry| *entry = ops[i].calculate_next(*entry, parsed))
                    .or_insert(parsed);
            }
            acc
        })
        .iter()
        .fold(0, |acc, next| acc + next.1)
        .to_string()
}

fn part_two(lines: Vec<String>) -> String {
    let ops: Vec<_> = lines
        .iter()
        .rev()
        .next()
        .unwrap()
        .split_whitespace()
        .map(|next| Ops::from(next))
        .collect();

    lines[..lines.len() - 1]
        .iter()
        .map(|next| next.split_whitespace())
        .fold(
            HashMap::new(),
            |mut acc: HashMap<usize, Vec<String>>, next| {
                for (i, num) in next.enumerate() {
                    let each_num: Vec<char> = num.chars().collect();
                    acc.entry(i)
                        .and_modify(|entry| {
                            for (j, string_entry) in entry.iter_mut().enumerate() {
                                if let Some(current_num) = each_num.get(j) {
                                    string_entry.push(*current_num);
                                }
                            }
                        })
                        .or_insert(each_num.iter().map(|c| c.to_string()).collect());
                }
                dbg!(&acc);
                acc
            },
        )
        .iter()
        .fold(0, |acc, next| {
            acc + next.1.iter().fold(0, |inner_acc, next_num| {
                ops[*next.0].calculate_next(inner_acc, next_num.parse().unwrap())
            })
        })
        .to_string()
}
