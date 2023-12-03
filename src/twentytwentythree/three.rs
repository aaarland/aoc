use crate::solutions::Solution;
pub struct DayThree;

impl Solution for DayThree {
    fn solve(&self, lines: Vec<String>) -> () {
        println!("Day 3");
        println!("Part one: {}", part_one(lines.clone()));
        println!("Part two: {}", part_two(lines.clone()));
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
    0
}

#[cfg(test)]
mod tests {
    use crate::utils::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = read_file(&"2023/example3".to_string());
        assert_eq!(part_one(lines), 4361);
    }

    #[test]
    fn test_part_two() {}
}
