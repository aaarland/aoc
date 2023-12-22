use std::{collections::VecDeque, fmt::Display};

use image::GenericImage;

use crate::{solutions::Solution, sprites};
pub struct DayEighteen;

impl Solution for DayEighteen {
    fn solve(&self, lines: Vec<String>) -> () {
        let part_one = part_one(lines.clone());
        println!("Part one: {}", part_one);
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
            map: vec![vec![Terrain::Ground; 600]; 600],
            width: 600,
            height: 600,
        }
    }
    fn dig(&mut self, pos: (usize, usize)) {
        self.width = self.width.max(pos.0 + 1);
        self.height = self.height.max(pos.1 + 1);
        if self.map.get(pos.1).is_none() {
            self.map.push(vec![Terrain::Ground; self.width]);
        }
        if self.map.get(pos.1).unwrap().get(pos.0).is_none() {
            self.map.get_mut(pos.1).unwrap().push(Terrain::Trench);
        } else {
            self.map.get_mut(pos.1).unwrap()[pos.0] = Terrain::Trench;
        }
    }

    fn fill(&mut self) -> Self {
        let mut new_map = self.map.clone();
        let start = self.find_start();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(start);
        while let Some(pos) = queue.pop_front() {
            if pos.0 >= self.width || pos.1 >= self.height {
                continue;
            }
            if new_map.get(pos.1).unwrap().get(pos.0).unwrap() == &Terrain::Trench {
                continue;
            }
            if self.map.get(pos.1).unwrap().get(pos.0).unwrap() == &Terrain::Trench {
                continue;
            }
            if self.map.get(pos.1).unwrap().get(pos.0).unwrap() == &Terrain::Ground {
                new_map.get_mut(pos.1).unwrap()[pos.0] = Terrain::Trench;
                queue.push_back((pos.0 + 1, pos.1));
                if pos.0 != 0 {
                    queue.push_back((pos.0 - 1, pos.1));
                }
                queue.push_back((pos.0, pos.1 + 1));
                if pos.1 != 0 {
                    queue.push_back((pos.0, pos.1 - 1));
                }
            }
        }
        DigPlan {
            map: new_map,
            width: self.width,
            height: self.height,
        }
    }

    fn find_start(&self) -> (usize, usize) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col == &Terrain::Trench {
                    return (x + 1, y + 1);
                }
            }
        }
        panic!("No start found");
    }
}

fn generate_image(dig_plan: &DigPlan) {
    let sprite_size = 16;
    let sprites = sprites::load_sprite("sprites/ground.png", sprite_size, 2, 1).unwrap();
    let height = dig_plan.height;
    let width = dig_plan.width;
    let mut image =
        image::DynamicImage::new_rgba8(width as u32 * sprite_size, height as u32 * sprite_size);
    let ground = &sprites[0];
    let trench = &sprites[1];


    println!("{} {}", width, height);
    for (y, row) in dig_plan.map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let sprite = match col {
                Terrain::Trench => trench,
                Terrain::Ground => ground,
            };
            image.copy_from(sprite, x as u32 * sprite_size, y as u32 * sprite_size).unwrap();
        }
    }
    image.save("day18.png").unwrap();
}
fn part_one(lines: Vec<String>) -> usize {
    let steps = lines.iter().map(|line| Step::from(line));
    let mut dig_plan = DigPlan::new();
    let mut pos = (250, 250);
    steps.for_each(|step| match step.direction {
        Direction::Up => {
            for _ in 0..step.count {
                match pos.1 {
                    0 => {
                        dig_plan
                            .map
                            .insert(0, vec![Terrain::Ground; dig_plan.width]);
                    }
                    _ => pos.1 -= 1,
                }
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
                match pos.0 {
                    0 => dig_plan.map.iter_mut().for_each(|row| {
                        row.insert(0, Terrain::Ground);
                    }),
                    _ => pos.0 -= 1,
                }
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

    println!("{}", dig_plan);
    // generate_image(&dig_plan);


    let new_map = dig_plan.fill();

    println!("{}", new_map);
    new_map
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

    #[test]
    fn test_part_one() {
        let lines = utils::read_file(&"2023/example18".into());
        assert_eq!(part_one(lines), 62);
    }

    #[test]
    fn test_part_two() {}
}
