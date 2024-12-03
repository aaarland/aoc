use crate::define_solution;

define_solution!(DayTwo, part_one, part_two);

fn part_one(lines: Vec<String>) -> String {
    lines
        .iter()
        .fold(0, |current, next| {
            let numbers = next
                .split_whitespace()
                .map(|num| num.parse::<i32>().expect("failed to parse"))
                .collect::<Vec<_>>();
            if check_num(&numbers) {
                current + 1
            } else {
                current
            }
        })
        .to_string()
}

fn check_num(numbers: &Vec<i32>) -> bool {
    if numbers[0] == numbers[1] {
        return false;
    };
    let direction = if numbers[0] > numbers[1] { 1 } else { -1 };

    for i in 0..numbers.len() - 1 {
        let diff = numbers[i] - numbers[i + 1];
        if (diff * direction) > 3 || (diff * direction) < 1 {
            return false;
        }
    }
    true
}

fn check_all(numbers: Vec<i32>) -> bool {
    for i in 0..numbers.len() {
        let mut numbers_copy = numbers.clone();
        let _removed_num = numbers_copy.remove(i);
        if check_num(&numbers_copy) {
            return true;
        }
    }
    false
}

fn part_two(lines: Vec<String>) -> String {
    lines
        .iter()
        .fold(0, |current, next| {
            let numbers = next
                .split_whitespace()
                .map(|num| num.parse::<i32>().expect("failed to parse"))
                .collect::<Vec<_>>();
            if check_num(&numbers) || check_all(numbers) {
                current + 1
            } else {
                current
            }
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = utils::read_file(&"2024/example2".into());
        assert_eq!(part_one(lines), "2");
    }

    #[test]
    fn test_part_two() {
        let lines = utils::read_file(&"2024/example2".into());
        assert_eq!(part_two(lines), "4");
    }
}
