use std::collections::HashMap;

use crate::define_solution;

define_solution!(DaySix, part_one, part_two);

#[derive(Debug, PartialEq)]
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

struct NewOps {
    ops: Ops,
    start_idx: usize,
}

fn part_two(lines: Vec<String>) -> String {
    let mut ops = vec![];
    for (i, c) in lines.iter().rev().next().unwrap().chars().enumerate() {
        if c == ' ' {
            continue;
        }
        ops.push(NewOps {
            ops: Ops::from(c.to_string().as_str()),
            start_idx: i,
        });
    }

    lines[..lines.len() - 1]
        .iter()
        .map(|next| {
            let mut problems = vec![];
            for index in 0..ops.len() {
                let current_range = if index + 1 == ops.len() {
                    (ops[index].start_idx, next.len())
                } else {
                    (ops[index].start_idx, ops[index + 1].start_idx - 1)
                };
                let problem: Vec<_> = next[current_range.0..current_range.1].chars().collect();
                problems.push(problem);
            }
            problems
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<usize, Vec<String>>, next| {
                for (i, num) in next.iter().enumerate() {
                    acc.entry(i)
                        .and_modify(|entry| {
                            for (j, string_entry) in entry.iter_mut().enumerate() {
                                if let Some(current_num) = num.get(j) {
                                    string_entry.push(*current_num);
                                }
                            }
                        })
                        .or_insert(num.iter().map(|c| c.to_string()).collect());
                }
                acc
            },
        )
        .iter()
        .fold(0, |acc, next| {
            acc + next.1.iter().fold(0, |inner_acc, next_num| {
                let op = &ops[*next.0];
                if op.ops == Multiply && inner_acc == 0 {
                op
                    .ops
                    .calculate_next(1, next_num.trim().parse().unwrap())

                }else {
                op
                    .ops
                    .calculate_next(inner_acc, next_num.trim().parse().unwrap())

                }
            })
        })
        .to_string()
}
