use std::cmp;

use crate::solutions::Solution;
pub struct DayTwo;

#[cfg(test)]
mod tests {
    use crate::utils::read_file;
    #[test]
    fn test_part_one() {
        let lines = read_file(&"2023/example2".to_string());
        assert_eq!(super::part_two(lines), 2286);
    }
}

impl Solution for DayTwo {
    fn solve(&self, lines: Vec<String>) -> () {
        println!("Total: {}", part_two(lines));
    }
}

fn part_two(lines: Vec<String>) -> i32 {
    let mut total = 0;
    lines.iter().for_each(|line| {
        let games = line
            .split(':')
            .nth(1)
            .unwrap()
            .split(';')
            .collect::<Vec<&str>>();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for game in games.iter() {
            game.split(',').for_each(|color| {
                let color = color.trim();
                let color = color.split(' ').collect::<Vec<&str>>();
                let count = color[0].parse::<i32>().unwrap();
                let color = Color::from_str(color[1]);
                match color {
                    Color::Red => {
                        min_red = cmp::max(min_red, count);
                    }
                    Color::Green => {
                        min_green = cmp::max(min_green, count);
                    }
                    Color::Blue => {
                        min_blue = cmp::max(min_blue, count);
                    }
                }
            });
        }
        total += min_red * min_green * min_blue;
    });
    total
}

#[allow(dead_code)]
fn part_one(color: Color, count: i32, max_red: i32, max_green: i32, max_blue: i32) -> bool {
    match color {
        Color::Red => {
            if count > max_red {
                return false;
            }
        }
        Color::Green => {
            if count > max_green {
                return false;
            }
        }
        Color::Blue => {
            if count > max_blue {
                return false;
            }
        }
    }
    true
}

impl Color {
    fn from_str(color: &str) -> Color {
        match color {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("Invalid color"),
        }
    }
}

enum Color {
    Red,
    Green,
    Blue,
}
