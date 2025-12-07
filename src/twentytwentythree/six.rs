use crate::solutions::{Part, Solution, UpdateFn};
pub struct DaySix;

impl Solution for DaySix {
    fn solve(&self, lines: Vec<String>, part: Part, _: Option<UpdateFn>) -> String {
        match part {
            Part::One => part_one(lines).to_string(),
            Part::Two => part_two(lines).to_string(),
        }
    }
}

fn part_one(lines: Vec<String>) -> i32 {
    let times = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let distance = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|d| d.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    times
        .iter()
        .zip(distance.iter())
        .map(|(t, d)| {
            let mut total_wins = 0;
            for i in 0..*t {
                let distance = i * (t - i);
                if distance > *d {
                    total_wins += 1;
                }
            }
            total_wins
        })
        .product::<i32>()
}

fn part_two(lines: Vec<String>) -> u64 {
    let time = lines[0]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let distance = lines[1]
        .split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<u64>()
        .unwrap();

    let mut first = 0;
    for i in 0..time {
        let current_distance = i * (time - i);
        if current_distance > distance {
            first = i;
            break;
        }
    }
    return time - (first * 2) + 1;
}

#[cfg(test)]
mod tests {
    use crate::utils::read_file;

    use super::*;

    #[tokio::test]
    async fn test_part_one() {
        let lines = read_file(&"2023/example6".to_string()).await;
        assert_eq!(part_one(lines), 288);
    }

    #[tokio::test]
    async fn test_part_two() {
        let lines = read_file(&"2023/example6".to_string()).await;
        assert_eq!(part_two(lines), 71503);
    }
}
