use std::collections::{HashSet, VecDeque};

use crate::solutions::{Part, Solution};
pub struct DayThree;

impl Solution for DayThree {
    fn solve(&self, lines: Vec<String>, part: Part) -> String {
        match part {
            Part::One => part_one(lines).to_string(),
            Part::Two => part_two(lines).to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum EngineScheme {
    Number(char),
    Symbol,
    Period,
    Gear,
}

impl From<char> for EngineScheme {
    fn from(value: char) -> Self {
        match value {
            '0' => EngineScheme::Number('0'),
            '1' => EngineScheme::Number('1'),
            '2' => EngineScheme::Number('2'),
            '3' => EngineScheme::Number('3'),
            '4' => EngineScheme::Number('4'),
            '5' => EngineScheme::Number('5'),
            '6' => EngineScheme::Number('6'),
            '7' => EngineScheme::Number('7'),
            '8' => EngineScheme::Number('8'),
            '9' => EngineScheme::Number('9'),
            '.' => EngineScheme::Period,
            '*' => EngineScheme::Gear,
            _ => EngineScheme::Symbol,
        }
    }
}

fn part_one(lines: Vec<String>) -> i32 {
    let mut total = 0;
    let mut numbers = Vec::new();
    let matrix = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| EngineScheme::from(c))
                .collect::<Vec<EngineScheme>>()
        })
        .collect::<Vec<Vec<EngineScheme>>>();
    for (i, line) in matrix.iter().enumerate() {
        let mut current_number = String::new();
        let mut is_valid_part = false;
        for (j, character) in line.iter().enumerate() {
            match character {
                EngineScheme::Number(num) => {
                    current_number.push(*num);
                    if (j + 1 < matrix[i].len()
                        && (matrix[i][j + 1] == EngineScheme::Symbol
                            || matrix[i][j + 1] == EngineScheme::Gear))
                        || (j > 0
                            && (matrix[i][j - 1] == EngineScheme::Symbol
                                || matrix[i][j - 1] == EngineScheme::Gear))
                        || (i + 1 < matrix.len()
                            && (matrix[i + 1][j] == EngineScheme::Symbol
                                || matrix[i + 1][j] == EngineScheme::Gear))
                        || (i > 0
                            && (matrix[i - 1][j] == EngineScheme::Symbol
                                || matrix[i - 1][j] == EngineScheme::Gear))
                        || (i + 1 < matrix.len()
                            && j + 1 < matrix[i].len()
                            && (matrix[i + 1][j + 1] == EngineScheme::Symbol
                                || matrix[i + 1][j + 1] == EngineScheme::Gear))
                        || (i > 0
                            && j > 0
                            && (matrix[i - 1][j - 1] == EngineScheme::Symbol
                                || matrix[i - 1][j - 1] == EngineScheme::Gear))
                        || (j > 0
                            && i + 1 < matrix.len()
                            && (matrix[i + 1][j - 1] == EngineScheme::Symbol
                                || matrix[i + 1][j - 1] == EngineScheme::Gear))
                        || (i > 0
                            && j + 1 < matrix[i].len()
                            && (matrix[i - 1][j + 1] == EngineScheme::Symbol
                                || matrix[i - 1][j + 1] == EngineScheme::Gear))
                    {
                        is_valid_part = true;
                    }
                }
                _ => {
                    current_number.clear();
                }
            }
            if is_valid_part
                && (j + 1 < matrix[i].len()
                    && (matrix[i][j + 1] == EngineScheme::Period
                        || matrix[i][j + 1] == EngineScheme::Symbol
                        || matrix[i][j + 1] == EngineScheme::Gear)
                    || j == matrix[i].len() - 1)
            {
                is_valid_part = false;

                match current_number.parse::<i32>() {
                    Ok(num) => {
                        total += num;
                        numbers.push(num);
                        num
                    }
                    Err(_) => 0,
                };
            }
        }
    }
    total
}

fn part_two(lines: Vec<String>) -> i32 {
    let mut total = 0;
    let matrix = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| EngineScheme::from(c))
                .collect::<Vec<EngineScheme>>()
        })
        .collect::<Vec<Vec<EngineScheme>>>();
    for (i, line) in matrix.iter().enumerate() {
        for (j, character) in line.iter().enumerate() {
            match character {
                EngineScheme::Gear => {
                    let mut gear_nums = HashSet::new();
                    if i == 0 || j == 0 || i == matrix.len() - 1 || j == matrix[i].len() - 1 {
                        continue;
                    }
                    check_and_add_number(&matrix, i - 1, j - 1, &mut gear_nums);
                    check_and_add_number(&matrix, i - 1, j, &mut gear_nums);
                    check_and_add_number(&matrix, i - 1, j + 1, &mut gear_nums);
                    check_and_add_number(&matrix, i, j - 1, &mut gear_nums);
                    check_and_add_number(&matrix, i, j + 1, &mut gear_nums);
                    check_and_add_number(&matrix, i + 1, j - 1, &mut gear_nums);
                    check_and_add_number(&matrix, i + 1, j, &mut gear_nums);
                    check_and_add_number(&matrix, i + 1, j + 1, &mut gear_nums);
                    if gear_nums.len() == 2 {
                        let gear_num = gear_nums.into_iter().product::<i32>();
                        total += gear_num;
                    }
                }
                _ => {}
            }
        }
    }
    total
}

fn check_and_add_number(
    matrix: &Vec<Vec<EngineScheme>>,
    i: usize,
    j: usize,
    gear_numbers: &mut HashSet<i32>,
) {
    if i < matrix.len() && j < matrix[i].len() {
        match matrix[i][j] {
            EngineScheme::Number(_) => {
                let mut local_nums = VecDeque::new();
                let mut local_j = j;
                while let EngineScheme::Number(next_num) = matrix[i][local_j] {
                    local_nums.push_front(next_num);
                    if local_j == 0 {
                        break;
                    }
                    local_j -= 1;
                }
                local_j = j + 1;
                while let EngineScheme::Number(next_num) = matrix[i][local_j] {
                    local_nums.push_back(next_num);
                    if local_j == matrix[i].len() - 1 {
                        break;
                    }
                    local_j += 1;
                }
                let gear_num = local_nums
                    .into_iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
                gear_numbers.insert(gear_num);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::read_file;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = read_file(&"2023/example3".to_string()).await;
        assert_eq!(part_one(lines), 4361);
    }

    #[tokio::test]
    async fn test_part_two() {
        let lines = read_file(&"2023/example3".to_string()).await;
        assert_eq!(part_two(lines), 467835);
    }
}
