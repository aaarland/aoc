
use crate::solutions::{Part, Solution};
pub struct DayFour;

impl Solution for DayFour {
    fn solve(&self, lines: Vec<String>, part: Part) -> String {
        match part {
            Part::One => part_one(lines).to_string(),
            Part::Two => part_two(lines).to_string(),
        }
    }
}

fn part_one(lines: Vec<String>) -> i32 {
    let mut total = 0;
    lines.iter().for_each(|line| {
        let numbers = line.split(':').nth(1).unwrap();
        let winning_numbers = numbers
            .split('|')
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let my_numbers = numbers
            .split('|')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut is_first = true;
        let mut current_score = 0;

        winning_numbers.iter().for_each(|num| {
            if my_numbers.contains(num) {
                if is_first {
                    current_score += 1;
                } else {
                    current_score *= 2;
                }
                is_first = false;
            }
        });
        total += current_score;
    });
    total
}

fn part_two(lines: Vec<String>) -> i32 {
    let mut played = vec![0; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        played[i] += 1;
        let numbers = line.split(':').nth(1).unwrap();
        let winning_numbers = numbers
            .split('|')
            .nth(0)
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let my_numbers = numbers
            .split('|')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut total_winnings = 0;
        winning_numbers.iter().for_each(|num| {
            if my_numbers.contains(num) {
                total_winnings += 1;
            }
        });
        for j in 0..total_winnings {
            played[i + j + 1] += played[i];
        }
    }
    played.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = utils::read_file(&"2023/example4".to_string());
        assert_eq!(part_one(lines), 13);
    }

    #[test]
    fn test_part_two() {
        let lines = utils::read_file(&"2023/example4".to_string());
        assert_eq!(part_two(lines), 30);
    }
}
