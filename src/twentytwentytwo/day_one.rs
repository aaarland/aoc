use crate::solutions::{Part, Solution};

pub struct DayOne;


impl Solution for DayOne {
    fn solve(&self, lines: Vec<String>, part: Part) -> String {
        solution(lines)
    }
}
//Start day 1
fn solution(lines: Vec<String>) -> String {
    let mut all_total_calories: Vec<i32> = vec![];
    let mut total_calories = 0;
    for line in lines {
        if line.is_empty() {
            all_total_calories.push(total_calories);
            total_calories = 0;
            continue;
        }
        total_calories += line.parse::<i32>().expect("Failed to parse");
    }
    all_total_calories.sort();
    total_calories = 0;
    let mut index = all_total_calories.len() - 1;

    while index >= all_total_calories.len() - 3 {
        total_calories += all_total_calories[index];
        print!("{} ", index);
        index -= 1;
    }
    total_calories.to_string()
}
//End day 1
