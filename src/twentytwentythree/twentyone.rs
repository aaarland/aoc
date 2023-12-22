use std::{collections::{VecDeque, HashSet}, fmt::Display};

use crate::solutions::Solution;
pub struct DayTwentyOne;

impl Solution for DayTwentyOne {
    fn solve(&self, lines: Vec<String>) -> () {
        let part_one = part_one(lines.clone(), 64);
        println!("Part one: {}", part_one);
    }
}

#[derive(Clone, PartialEq)]
enum Plots {
    Garden,
    Rock,
}

struct Elf {
    grid: Vec<Vec<Plots>>,
    steps: HashSet<(usize, usize)>,
    current_step: usize,
}

impl Display for Elf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                s.push(match self.grid[y][x] {
                    Plots::Garden => {
                        if self.steps.contains(&(x, y)) {
                            'O'
                        } else {
                            '.'
                        }
                    }
                    Plots::Rock => '#',
                });
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

impl Elf {
    fn new(grid: Vec<Vec<Plots>>) -> Self {
        Elf {
            grid,
            steps: HashSet::new(),
            current_step: 0,
        }
    }

    fn walk(&mut self, x: usize, y: usize, total_steps: usize) {
        self.steps.insert((x, y));
        while self.current_step < total_steps {
            let mut new_steps = HashSet::new();
            for (x, y) in self.steps.clone().into_iter() {
                new_steps.extend(self.add_steps(x, y));
            }
            self.steps = new_steps;
            self.current_step += 1;
        }
    }

    fn add_steps(&self, x: usize, y: usize) -> HashSet<(usize, usize)> {
        let mut new_steps = HashSet::new();
        if self.grid[y][x] == Plots::Rock {
            return new_steps;
        }
        if x > 0 {
            new_steps.insert((x - 1, y));
        }
        if y > 0 {
            new_steps.insert((x, y - 1));
        }
        if x < self.grid[0].len() - 1 {
            new_steps.insert((x + 1, y));
        }
        if y < self.grid.len() - 1 {
            new_steps.insert((x, y + 1));
        }
        new_steps
    }
}

fn part_one(lines: Vec<String>, total_steps: usize) -> usize {
    let mut start = None;
    let grid = lines
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Plots::Garden,
                    '#' => Plots::Rock,
                    'S' => {
                        start = Some((x, y));
                        Plots::Garden
                    }
                    _ => panic!("Invalid input"),
                })
                .collect::<Vec<Plots>>()
        })
        .collect::<Vec<Vec<Plots>>>();
    let mut elf = Elf::new(grid);
    let (x, y) = start.unwrap();
    elf.walk(x, y, total_steps);
    println!("{}", elf);
    println!("{:?}", elf.steps);
    elf.steps.len()
}

fn part_two() {}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = utils::read_file(&"2023/example21".into());
        let part_one = part_one(lines.clone(), 6);
        assert_eq!(part_one, 16);
    }

    #[test]
    fn test_part_two() {}
}
