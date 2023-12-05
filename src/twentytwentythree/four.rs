use std::cmp;

use crate::solutions::Solution;
pub struct DayFour;

impl Solution for DayFour {
    fn solve(&self, lines: Vec<String>) -> () {
        println!("Day Four");
        let time = std::time::Instant::now();
        let part_one = part_one(lines.clone());
        let elapsed = time.elapsed();
        println!("Part one: {} ({:?})", part_one, elapsed);
        let time = std::time::Instant::now();
        let part_two = part_two(lines.clone());
        let elapsed = time.elapsed();
        println!("Part two: {} ({:?})", part_two, elapsed);
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

fn add_copies(
    lines: &Vec<String>,
    line: &String,
    total_cards: &mut Vec<String>,
    i: usize,
) -> Vec<String> {
    let mut copies = Vec::new();
    total_cards.push(line.clone());
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
    let mut copies_index = i + 1;
    for (j, num) in winning_numbers.iter().enumerate() {
        if my_numbers.contains(num) {
            copies.push(lines[copies_index].clone());
            copies_index += 1;
        }
    }
    println!("Line {} has {} copies", line, copies.len());
    copies
}
fn part_two(lines: Vec<String>) -> i32 {
    let mut total_cards = Vec::new();
    let mut copies: Vec<String> = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let mut new_copies = add_copies(&lines, line, &mut total_cards, i);
        copies.append(&mut new_copies);
    }

    copies.iter().for_each(|card| {
        println!("{}", card);
    });
    for i in 0..copies.len() {
        let index = &copies[i]
            .chars()
            .skip_while(|c| !c.is_digit(10))
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let mut new_copies = add_copies(&copies, &lines[index - 1], &mut total_cards, i);
        copies.append(&mut new_copies);
    }
    total_cards.iter().for_each(|card| {
        println!("{}", card);
    });

    total_cards.len() as i32
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
