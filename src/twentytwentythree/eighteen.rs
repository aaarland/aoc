use std::fmt::Display;

use crate::solutions::{Part, Solution};
pub struct DayEighteen;

impl Solution for DayEighteen {
    fn solve(&self, lines: Vec<String>, part: Part) -> String {
        part_one(lines.clone()).to_string()
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Step {
    direction: Direction,
    count: usize,
    hex: String,
}

impl From<&String> for Step {
    fn from(s: &String) -> Self {
        let mut split = s.split_whitespace();
        let direction = match split.next().unwrap() {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("Invalid direction"),
        };
        let count = split.next().unwrap().parse::<usize>().unwrap();
        let hex = split.next().unwrap().to_string();
        Step {
            direction,
            count,
            hex,
        }
    }
}
#[derive(Clone, PartialEq)]
enum Terrain {
    Trench,
    Ground,
}

struct DigPlan {
    map: Vec<Vec<Terrain>>,
    width: usize,
    height: usize,
}
impl Display for DigPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for row in self.map.iter() {
            for col in row.iter() {
                s.push(match col {
                    Terrain::Trench => '#',
                    Terrain::Ground => '.',
                });
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl DigPlan {
    fn new() -> Self {
        DigPlan {
            map: vec![vec![Terrain::Trench]],
            width: 0,
            height: 0,
        }
    }
    fn dig(&mut self, pos: (usize, usize)) {
        self.width = self.width.max(pos.0 + 1);
        self.height = self.height.max(pos.1 + 1);
        println!("Digging {:?}", pos);
        println!("Width: {}, Height: {}", self.width, self.height);
        if self.map.get(pos.1).is_none() {
            self.map.push(vec![Terrain::Ground; self.width]);
        }
        if self.map.get(pos.1).unwrap().get(pos.0).is_none() {
            self.map.get_mut(pos.1).unwrap().push(Terrain::Trench);
        } else {
            self.map.get_mut(pos.1).unwrap()[pos.0] = Terrain::Trench;
        }
    }
}

fn part_one(lines: Vec<String>) -> usize {
    let steps = lines.iter().map(|line| Step::from(line));
    let mut dig_plan = DigPlan::new();
    let mut pos = (0, 0);
    steps.for_each(|step| match step.direction {
        Direction::Up => {
            for _ in 0..step.count {
                if pos.1 == 0 {
                    panic!("Invalid position");
                }
                pos.1 -= 1;
                dig_plan.dig(pos);
            }
        }
        Direction::Down => {
            for _ in 0..step.count {
                pos.1 += 1;
                dig_plan.dig(pos);
            }
        }
        Direction::Left => {
            for _ in 0..step.count {
                if pos.0 == 0 {
                    panic!("Invalid position");
                }
                pos.0 -= 1;
                dig_plan.dig(pos);
            }
        }
        Direction::Right => {
            for _ in 0..step.count {
                pos.0 += 1;
                dig_plan.dig(pos);
            }
        }
    });
    dig_plan.map.iter_mut().for_each(|row| {
        row.iter_mut()
            .skip_while(|t| **t == Terrain::Ground)
            .skip_while(|t| **t == Terrain::Trench)
            .take_while(|t| **t == Terrain::Ground)
            .for_each(|t| *t = Terrain::Trench);
    });

    println!("{}", dig_plan);
    dig_plan
        .map
        .iter()
        .map(|row| row.iter().filter(|t| **t == Terrain::Trench).count())
        .sum()
}

fn part_two() {}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = utils::read_file(&"2023/example18".into()).await;
        assert_eq!(part_one(lines), 62);
    }

    #[tokio::test]
    async fn test_part_two() {}
}
