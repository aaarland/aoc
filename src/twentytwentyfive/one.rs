
use num_traits::abs;

use crate::define_solution;

define_solution!(DayOne, part_one, part_two);

fn part_one(lines: Vec<String>) -> String {
    let mut password = 0;
    let mut result = 50;
    for line in lines {
        let mut chars = line.chars();
        let left_or_right = chars.next().unwrap();
        let multiplication = match left_or_right {
            'L' => -1,
            'R' => 1,
            _ => panic!("SHOULD BE L OR R"),
        };
        let number: i32 = chars.as_str().parse().unwrap();
        result = (result + (multiplication * number) + 100) % 100;
        if result == 0 {
            password += 1;
        }
    }
    password.to_string()
}
fn part_two(lines: Vec<String>) -> String {
    let mut password = 0;
    let mut result = 50;
    for line in lines {
        let prev = result;
        let mut chars = line.chars();
        let left_or_right = chars.next().unwrap();
        let multiplication = match left_or_right {
            'L' => -1,
            'R' => 1,
            _ => panic!("SHOULD BE L OR R"),
        };
        let number: i32 = chars.as_str().parse().unwrap();
        result += multiplication * number;
        if result == 0 || prev * result < 0{
            password += 1;
        }
        password += abs(result / 100);
        result %= 100;

    }
    password.to_string()
}
