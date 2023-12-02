use std::cmp;

use crate::solutions::Solution;
pub struct DayTwo;

impl Solution for DayTwo {
    fn solve(&self, lines: Vec<String>) -> () {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let mut total = 0;

        lines.iter().for_each(|line| {
            let mut is_game_possible = true;
            let current_game_id = line
                .chars()
                .skip_while(|c| !c.is_digit(10))
                .take_while(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
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
        println!("Total: {}", total);
    }
}


fn part_one(
    color: Color,
    count: i32,
    max_red: i32,
    max_green: i32,
    max_blue: i32,
) -> bool{
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
