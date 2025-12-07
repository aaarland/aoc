use std::collections::HashMap;

use crate::define_solution;

define_solution!(DayFive, part_one, part_two);

fn part_one(lines: Vec<String>) -> String {
    let first_section = lines.iter().take_while(|line| !line.is_empty());
    let second_section = lines.iter().skip_while(|line| !line.is_empty()).skip(1);
    let (_before, after): (HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) = first_section.fold(
        (HashMap::new(), HashMap::new()),
        |(mut before, mut after), next| {
            let (x_str, y_str) = next.split_once('|').expect("Unable to split");
            let x = x_str.parse().expect("couldn't parse {x_str}");
            let y = y_str.parse().expect("couldn't parse {y_str}");
            before
                .entry(x)
                .and_modify(|pages| pages.push(y))
                .or_insert(vec![y]);
            after
                .entry(y)
                .and_modify(|pages| pages.push(x))
                .or_insert(vec![x]);

            (before, after)
        },
    );
    let good = second_section.fold(Vec::new(), |mut current, next| {
        let mut numbers_to_check: Vec<i32> = next
            .split(',')
            .map(|num| num.parse().expect("Failed to parse {num}"))
            .collect();
        if check_numbers::<fn(&mut Vec<i32>, usize)>(&mut numbers_to_check, &after, None) {
            current.push(numbers_to_check);
        }
        current
    });
    good.iter()
        .fold(0, |acc, next| acc + next[next.len() / 2])
        .to_string()
}

fn check_numbers<F>(
    numbers_to_check: &mut Vec<i32>,
    after: &HashMap<i32, Vec<i32>>,
    swap: Option<F>,
) -> bool
where
    F: Fn(&mut Vec<i32>, usize),
{
    for (i, number) in numbers_to_check.iter().enumerate() {
        let maybe_checking = after.get(number);
        let rest = &numbers_to_check[i..];
        match maybe_checking {
            Some(checking) if checking.iter().any(|c| rest.contains(c)) => {
                println!("numbers {rest:?} contains {checking:?}");
                if let Some(swap_fn) = swap {
                    swap_fn(numbers_to_check, i);
                };
                println!("numbers_to_check {numbers_to_check:?}");
                return false;
            }
            _ => {}
        }
    }
    return true;
}

fn part_two(lines: Vec<String>) -> String {
    let first_section = lines.iter().take_while(|line| !line.is_empty());
    let second_section = lines.iter().skip_while(|line| !line.is_empty()).skip(1);
    let (_before, after): (HashMap<i32, Vec<i32>>, HashMap<i32, Vec<i32>>) = first_section.fold(
        (HashMap::new(), HashMap::new()),
        |(mut before, mut after), next| {
            let (x_str, y_str) = next.split_once('|').expect("Unable to split");
            let x = x_str.parse().expect("couldn't parse {x_str}");
            let y = y_str.parse().expect("couldn't parse {y_str}");
            before
                .entry(x)
                .and_modify(|pages| pages.push(y))
                .or_insert(vec![y]);
            after
                .entry(y)
                .and_modify(|pages| pages.push(x))
                .or_insert(vec![x]);

            (before, after)
        },
    );
    let good = second_section.fold(Vec::new(), |mut current, next| {
        let mut numbers_to_check: Vec<i32> = next
            .split(',')
            .map(|num| num.parse().expect("Failed to parse {num}"))
            .collect();
        if !check_numbers(
            &mut numbers_to_check,
            &after,
            Some(|nums: &mut Vec<i32>, index| nums.swap(index, index + 1)),
        ) {
            current.push(numbers_to_check);
        }
        current
    });
    good.iter()
        .fold(0, |acc, next| acc + next[next.len() / 2])
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = utils::read_db(5, 2024, &"example".into()).await;
        assert_eq!(part_one(lines), "143");
    }

    #[tokio::test]
    #[ignore = "not done"]
    async fn test_part_two() {
        let lines = utils::read_db(5, 2024, &"example".into()).await;
        assert_eq!(part_two(lines), "123");
    }
}
