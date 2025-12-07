use std::slice::Iter;
use std::usize;

use crate::define_solution;
use crate::twentytwentyfour::four::Direction::{
    East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
};

define_solution!(DayFour, part_one, part_two);

const XMAS: &str = "XMAS";
const XMAS_LEN: i32 = 4;

#[derive(Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
        ];
        DIRECTIONS.iter()
    }
}

fn search(
    direction: &Direction,
    lines: &Vec<String>,
    x: i32,
    y: i32,
) -> bool {
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let out_of_bounds = match direction {
        North => y - XMAS_LEN + 1 < 0,
        NorthEast => y - XMAS_LEN + 1 < 0 || x + XMAS_LEN > width,
        East => x + XMAS_LEN > width,
        SouthEast => y + XMAS_LEN > height || x + XMAS_LEN > width,
        South => y + XMAS_LEN > height,
        SouthWest => y + XMAS_LEN > height || x - XMAS_LEN + 1 < 0,
        West => x - XMAS_LEN + 1 < 0,
        NorthWest => y - XMAS_LEN + 1 < 0 || x - XMAS_LEN + 1 < 0,
    };

    if out_of_bounds {
        return false;
    };
    let x_direction = match direction {
        NorthEast => 1,
        East => 1,
        SouthEast => 1,
        SouthWest => -1,
        West => -1,
        NorthWest => -1,
        _ => 0,
    };
    let y_direction = match direction {
        North => -1,
        NorthEast => -1,
        SouthEast => 1,
        South => 1,
        SouthWest => 1,
        NorthWest => -1,
        _ => 0,
    };

    for i in 1..XMAS_LEN {
        let current_y = (y + (i * y_direction)) as usize;
        let current_x = (x + (i * x_direction)) as usize;
        let checking = XMAS.chars().nth(i as usize).expect("Out of bounds");
        let current_char = lines[current_y]
            .chars()
            .nth(current_x)
            .expect("Out of bounds");
        if checking != current_char {
            return false;
        };
    }
    true
}
fn part_one(lines: Vec<String>) -> String {
    let result = lines
        .iter()
        .enumerate()
        .fold(0, |current, (y, line)| {
            current
                + line.chars().enumerate().fold(0, |current_inner, (x, c)| {
                    if c == 'X' {
                        current_inner
                            + Direction::iterator().fold(0, |inner_inner, dir| {
                                match search(dir, &lines, x as i32, y as i32)
                                {
                                    true => inner_inner + 1,
                                    false => inner_inner,
                                }
                            })
                    } else {
                        current_inner
                    }
                })
        })
        .to_string();
    result
}

fn part_two(_lines: Vec<String>) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = utils::read_file(&"2024/example4".into()).await;
        //let fake = "MMMSXXMASM";
        assert_eq!(part_one(lines), "18");
    }

    #[tokio::test]
    async fn test_part_two() {}
}
