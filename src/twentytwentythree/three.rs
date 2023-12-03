use nom::Parser;

use crate::solutions::Solution;
pub struct DayThree;

impl Solution for DayThree {
    fn solve(&self, lines: Vec<String>) -> () {}
}

enum EngineScheme {
    Number(char),
    Symbol,
    Period,
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
            _ => EngineScheme::Symbol,
        }
    }
}

fn part_one(lines: Vec<String>) -> i32 {
    let width = lines[0].len();
    let height = lines.len();
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
                EngineScheme::Number(num) => {
                    let mut count = 0;
                    if i > 0 && j > 0 {
                        if let EngineScheme::Number(_) = matrix[i - 1][j - 1] {
                            count += 1;
                        }
                    }
                    if i > 0 {
                        if let EngineScheme::Number(_) = matrix[i - 1][j] {
                            count += 1;
                        }
                    }
                    if i > 0 && j < width - 1 {
                        if let EngineScheme::Number(_) = matrix[i - 1][j + 1] {
                            count += 1;
                        }
                    }
                    if j > 0 {
                        if let EngineScheme::Number(_) = matrix[i][j - 1] {
                            count += 1;
                        }
                    }
                    if j < width - 1 {
                        if let EngineScheme::Number(_) = matrix[i][j + 1] {
                            count += 1;
                        }
                    }
                    if i < height - 1 && j > 0 {
                        if let EngineScheme::Number(_) = matrix[i + 1][j - 1] {
                            count += 1;
                        }
                    }
                    if i < height - 1 {
                        if let EngineScheme::Number(_) = matrix[i + 1][j] {
                            count += 1;
                        }
                    }
                    if i < height - 1 && j < width - 1 {
                        if let EngineScheme::Number(_) = matrix[i + 1][j + 1] {
                            count += 1;
                        }
                    }
                    if count == 0 {
                        println!("{} {}", i, j);
                    }
                },
                EngineScheme::Symbol => {}
                EngineScheme::Period => {}
                _ => {}
            }
        }
    }
    0
}

fn part_two() {}

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
