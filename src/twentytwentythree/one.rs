use crate::solutions::Solution;

pub struct DayOne;

impl Solution for DayOne {
    fn solve(&self, lines: Vec<String>) -> () {
        let mut numbers: Vec<(char, char)> = Vec::new();
        lines.iter().for_each(|line| {
            let mut first_number = None;
            for (i, c) in line.chars().enumerate() {
                if let Some(_) = c.to_digit(10) {
                    first_number = Some(c);
                    break;
                } else {
                    first_number = match parse_numbers_as_strings(line.to_owned(), i) {
                        Some(number) => Some(number.to_char()),
                        None => None,
                    };
                    if let Some(_) = first_number {
                        break;
                    }
                }
            }
            let mut second_number = None;
            for (i, c) in line.chars().rev().enumerate() {
                if let Some(_) = c.to_digit(10) {
                    second_number = Some(c);
                    break;
                } else {
                    second_number = match parse_numbers_as_strings(line.to_owned(), line.len() - i - 1)
                    {
                        Some(number) => Some(number.to_char()),
                        None => None,
                    };
                    if let Some(_) = second_number {
                        break;
                    }
                }
            }
            numbers.push((first_number.unwrap_or('0'), second_number.unwrap_or('0')));
        });
        let mut total = 0;
        numbers.iter().for_each(|(first, second)| {
            if let Ok(number) = format!("{}{}", first, second).parse::<i32>() {
                total += number;
            }
        });
        println!("Total: {}", total);
    }
}

fn parse_numbers_as_strings(line: String, i: usize) -> Option<Numbers> {
    if i + 5 <= line.len() {
        if let Some(number) = Numbers::from_string(&line[i..i + 5]) {
            return Some(number);
        }
    }
    if i + 4 <=line.len() {
        if let Some(number) = Numbers::from_string(&line[i..i + 4]) {
            return Some(number);
        }
    }
    if i + 3 <= line.len() {
        if let Some(number) = Numbers::from_string(&line[i..i + 3]) {
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
