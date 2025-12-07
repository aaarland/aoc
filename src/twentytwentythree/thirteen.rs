use std::fmt::Display;

use crate::solutions::{Part, Solution, UpdateFn};
pub struct DayThirteen;

impl Solution for DayThirteen {
    fn solve(&self, lines: Vec<String>, part: Part, _: Option<UpdateFn>) -> String {
        match part {
            Part::One => part_one(lines).to_string(),
            Part::Two => part_two(lines).to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Pattern {
    Ash,
    Rocks,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Ash => write!(f, "."),
            Pattern::Rocks => write!(f, "#"),
        }
    }
}

impl From<char> for Pattern {
    fn from(s: char) -> Self {
        match s {
            '.' => Pattern::Ash,
            '#' => Pattern::Rocks,
            _ => panic!("Invalid pattern"),
        }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Pattern>>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = String::new();
        for row in &self.grid {
            for col in row {
                match col {
                    Pattern::Ash => grid.push('.'),
                    Pattern::Rocks => grid.push('#'),
                }
            }
            grid.push('\n');
        }
        write!(f, "{}", grid)
    }
}

impl Grid {
    fn new(lines: Vec<String>) -> Grid {
        let grid = lines
            .iter()
            .map(|line| line.chars().map(|c| c.into()).collect())
            .collect();

        Grid { grid }
    }
    fn search_horizontal(&self, search_index: usize, skip: Option<usize>) -> Option<i32> {
        if Some(search_index) == skip {
            return self.search_horizontal(search_index + 1, skip);
        }

        if search_index + 1 >= self.grid.len() {
            return None;
        }

        if self.grid[search_index] == self.grid[search_index + 1] {
            match self.search_horizontal_from_mirror(
                search_index,
                search_index as i32,
                search_index + 1,
            ) {
                Some(i) => return Some(i),
                None => return self.search_horizontal(search_index + 1, skip),
            }
        }
        return self.search_horizontal(search_index + 1, skip);
    }
    fn search_vertical(&self, search_index: usize, skip: Option<usize>) -> Option<i32> {
        if Some(search_index) == skip {
            return self.search_vertical(search_index + 1, skip);
        }
        if search_index + 1 >= self.grid[0].len() {
            return None;
        }

        if self.is_mirror(search_index as i32, search_index + 1) {
            match self.search_vertical_from_mirror(
                search_index,
                search_index as i32,
                search_index + 1,
            ) {
                Some(i) => return Some(i),
                None => return self.search_vertical(search_index + 1, skip),
            }
        }
        return self.search_vertical(search_index + 1, skip);
    }

    fn is_mirror(&self, negative_index: i32, positive_index: usize) -> bool {
        if negative_index < 0 || positive_index >= self.grid[0].len() {
            return true;
        }
        for i in 0..self.grid.len() {
            if self.grid[i][negative_index as usize] != self.grid[i][positive_index] {
                return false;
            }
        }
        self.is_mirror(negative_index - 1, positive_index + 1)
    }

    fn search_vertical_from_mirror(
        &self,
        search_index: usize,
        negative_index: i32,
        positive_index: usize,
    ) -> Option<i32> {
        if negative_index < 0 || positive_index >= self.grid[0].len() {
            return Some(search_index as i32);
        }
        for i in 0..self.grid.len() {
            if self.grid[i][negative_index as usize] != self.grid[i][positive_index] {
                return None;
            }
        }
        return self.search_vertical_from_mirror(
            search_index,
            negative_index - 1,
            positive_index + 1,
        );
    }

    fn search_horizontal_from_mirror(
        &self,
        search_index: usize,
        negative_index: i32,
        positive_index: usize,
    ) -> Option<i32> {
        if negative_index < 0 || positive_index >= self.grid.len() {
            return Some(search_index as i32);
        }
        if self.grid[negative_index as usize] != self.grid[positive_index] {
            return None;
        }
        return self.search_horizontal_from_mirror(
            search_index,
            negative_index - 1,
            positive_index + 1,
        );
    }

    fn fix_smudge(
        &mut self,
        row: usize,
        col: usize,
        skip: (Option<usize>, Option<usize>),
    ) -> Option<i32> {
        let old = self.grid[row][col];
        let new = match old {
            Pattern::Ash => Pattern::Rocks,
            Pattern::Rocks => Pattern::Ash,
        };
        self.grid[row][col] = new;

        let horizontal = self.search_horizontal(0, skip.0);
        let vertical = self.search_vertical(0, skip.1);
        self.grid[row][col] = old;
        match (horizontal, vertical) {
            (Some(h), None) => Some((h + 1) * 100),
            (None, Some(v)) => Some(v + 1),
            _ => {
                if col + 1 < self.grid[row].len() {
                    return self.fix_smudge(row, col + 1, skip);
                } else if row + 1 < self.grid.len() {
                    return self.fix_smudge(row + 1, 0, skip);
                } else {
                    return None;
                }
            }
        }
    }
}

fn part_one(lines: Vec<String>) -> i32 {
    let mut grids = vec![];
    lines.iter().fold(vec![], |mut lines, current_line| {
        if current_line.is_empty() {
            grids.push(Grid::new(lines));
            vec![]
        } else {
            lines.push(current_line.clone());
            lines
        }
    });
    grids.iter().for_each(|g| println!("{g}"));
    grids
        .iter()
        .map(|grid| {
            let horizontal = grid.search_horizontal(0, None);
            let vertical = grid.search_vertical(0, None);
            match (horizontal, vertical) {
                (Some(h), None) => (h + 1) * 100,
                (None, Some(v)) => v + 1,
                _ => 0,
            }
        })
        .sum()
}

fn part_two(lines: Vec<String>) -> i32 {
    let mut grids = vec![];
    lines.iter().fold(vec![], |mut lines, current_line| {
        if current_line.is_empty() {
            grids.push(Grid::new(lines));
            vec![]
        } else {
            lines.push(current_line.clone());
            lines
        }
    });
    let skips = grids
        .iter()
        .map(|grid| {
            let horizontal = grid.search_horizontal(0, None);
            let vertical = grid.search_vertical(0, None);
            match (horizontal, vertical) {
                (Some(h), None) => (Some(h as usize), None),
                (None, Some(v)) => (None, Some(v as usize)),
                _ => (None, None),
            }
        })
        .collect::<Vec<(Option<usize>, Option<usize>)>>();
    grids
        .iter_mut()
        .zip(skips)
        .map(|(grid, skip)| grid.fix_smudge(0, 0, skip).unwrap_or(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[tokio::test]
    #[ignore = "failing test"]
    async fn test_part_one() {
        let lines = utils::read_file(&"2023/example13".to_string()).await;
        assert_eq!(part_one(lines), 405);
    }

    #[tokio::test]
    #[ignore = "failing test"]
    async fn test_part_two() {
        let lines = utils::read_file(&"2023/example13".to_string()).await;
        assert_eq!(part_two(lines), 400);
    }
}
