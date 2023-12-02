use std::cmp;

use crate::solutions::Solution;

pub struct DayOne;

impl Solution for DayOne {
    fn solve(&self, lines: Vec<String>) -> () {
        let mut numbers: Vec<(char, char)> = Vec::new();
        lines.iter().for_each(|line| {
            let first_number = get_number_from_line(line.chars(), line.to_owned(), None);
            let second_number =
                get_number_from_line(line.chars().rev(), line.to_owned(), Some(line.len() - 1));
            numbers.push((first_number.unwrap_or('0'), second_number.unwrap_or('0')));
        });
        let mut total = 0;
        numbers.iter().for_each(|(first, second)| {
            println!("{}{}", first, second);
            if let Ok(number) = format!("{}{}", first, second).parse::<i32>() {
                total += number;
            }
        });
        println!("Total: {}", total);
    }
}

fn get_number_from_line(
    e: impl Iterator<Item = char>,
    line: String,
    max: Option<usize>,
) -> Option<char> {
    for (i, c) in e.enumerate() {
        if let Some(_) = c.to_digit(10) {
            return Some(c);
        } else {
            let local_idx = match max {
                Some(max) => max - i,
                None => i,
            };
            match parse_numbers_as_strings(line.to_owned(), local_idx) {
                Some(number) => return Some(number.to_char()),
                None => None::<char>,
            };
        }
    }
    None
}

fn parse_numbers_as_strings(line: String, i: usize) -> Option<Numbers> {
    for idx in 0..3 {
        let end = cmp::min(i + 5 - idx, line.len());
        println!("{}", &line[i..end]);
        if let Some(number) = Numbers::from_string(&line[i..end]) {
            return Some(number);
        }
    }
    return None;
}
impl Numbers {
    fn from_string(string: &str) -> Option<Numbers> {
        match string {
            "one" => Some(Numbers::One),
            "two" => Some(Numbers::Two),
            "three" => Some(Numbers::Three),
            "four" => Some(Numbers::Four),
            "five" => Some(Numbers::Five),
            "six" => Some(Numbers::Six),
            "seven" => Some(Numbers::Seven),
            "eight" => Some(Numbers::Eight),
            "nine" => Some(Numbers::Nine),
            "zero" => Some(Numbers::Zero),
            _ => None,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Numbers::One => '1',
            Numbers::Two => '2',
            Numbers::Three => '3',
            Numbers::Four => '4',
            Numbers::Five => '5',
            Numbers::Six => '6',
            Numbers::Seven => '7',
            Numbers::Eight => '8',
            Numbers::Nine => '9',
            Numbers::Zero => '0',
        }
    }
}

enum Numbers {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
}
