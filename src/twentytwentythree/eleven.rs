use std::{collections::HashMap, fmt::Display, cmp};

use crate::solutions::Solution;
use rayon::prelude::*;
pub struct DayEleven;

impl Solution for DayEleven {
    fn solve(&self, lines: Vec<String>) -> () {
        let time = std::time::Instant::now();
        let part_one = part_one(lines.clone());
        let elapsed = time.elapsed();
        println!("Part one: {}", part_one);
        println!("Elapsed: {:?}", elapsed);
    }
}

fn part_one(lines: Vec<String>) -> usize {
    let universe = Universe::new(lines);

    println!("width: {}, height: {}", universe.width, universe.height);
    println!("{:?}", universe.paths);
    println!("{}", universe);
    universe.paths.par_iter().map(|(_, v)| *v).sum()
}

fn part_two() {}

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<Spots>>,
    paths: HashMap<(Spots, Spots), usize>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Spots {
    Space,
    Galaxy(usize),
}

impl Display for Spots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Spots::Space => write!(f, "."),
            Spots::Galaxy(n) => write!(f, "{}", n),
        }
    }
}

impl Universe {
    fn new(lines: Vec<String>) -> Self {
        let mut galaxy_count = 0;
        println!("Creating grid");
        let mut grid: Vec<Vec<Spots>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Spots::Space,
                        '#' => {
                            galaxy_count += 1;
                            Spots::Galaxy(galaxy_count as usize)
                        }
                        _ => panic!("Invalid character"),
                    })
                    .collect()
            })
            .collect();
        println!("Finding Galaxies");
        let galaxyes = grid.iter().flatten().filter(|c| match **c {
            Spots::Galaxy(_) => true,
            _ => false,
        }).collect::<Vec<&Spots>>();
        let mut pairs = Vec::new();
        for i in 0..galaxyes.len() {
            for j in i+1..galaxyes.len() {
                pairs.push((*galaxyes[i], *galaxyes[j]));
            }
        }
        println!("Adding space");
        Self::add_space(&mut grid);
        let width = grid[0].len();
        let height = grid.len();


        println!("Finding paths");
        let paths = Self::get_paths(&grid,pairs);

        Self {
            grid,
            paths,
            width,
            height,
        }
    }

    fn add_space(grid: &mut Vec<Vec<Spots>>) {
        let mut width = grid[0].len();
        let mut height = grid.len();
        let mut x = 0;
        let mut y = 0;
        while y < height {
            if grid[y].iter().all(|c| *c == Spots::Space) {
                grid.insert(y, vec![Spots::Space; width]);
                y += 1;
                height += 1;
            }
            y += 1;
        }

        while x < width {
            if grid.iter().all(|line| line[x] == Spots::Space) {
                for i in 0..height {
                    grid[i].insert(x, Spots::Space);
                }
                x += 1;
                width += 1;
            }
            x += 1;
        }
    }

    fn get_paths(
        grid: &Vec<Vec<Spots>>,
        pairs: Vec<(Spots, Spots)>,
    ) -> HashMap<(Spots, Spots), usize> {
        pairs.par_iter().map(|(a, b)| {
            ((*a, *b), Self::get_shortest_path(grid, *a, *b))
        }).collect()
    }

    fn get_shortest_path(grid: &Vec<Vec<Spots>>, a: Spots, b: Spots) -> usize {
        let (start_y, line) = grid.iter().enumerate().find(|(_, line)| line.contains(&a)).unwrap();
        let start_x = line.iter().position(|c| *c == a).unwrap();
        let (end_y, line) = grid.iter().enumerate().find(|(_, line)| line.contains(&b)).unwrap();
        let end_x = line.iter().position(|c| *c == b).unwrap();
        let mut queue = Vec::new();
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut weights = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
        weights[start_y][start_x] = 0;

        queue.push((start_x, start_y, 0));
        while !queue.is_empty() {
            let (x, y, steps) = queue.remove(0);
            if x == end_x && y == end_y {
                return steps;
            }
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;
            weights[y][x] = cmp::min(steps, weights[y][x]);
            if x > 0 {
                queue.push((x - 1, y, steps + 1));
            }
            if x < grid[0].len() - 1{
                queue.push((x + 1, y, steps + 1));
            }
            if y > 0 {
                queue.push((x, y - 1, steps + 1));
            }
            if y < grid.len() - 1 {
                queue.push((x, y + 1, steps + 1));
            }
        }
        panic!("No path found");
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.grid {
            for c in line {
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = utils::read_file(&"2023/example11".to_string());
        assert_eq!(part_one(lines), 374);
    }

    #[test]
    fn test_part_two() {}
}
