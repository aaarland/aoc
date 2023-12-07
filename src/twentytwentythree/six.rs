use crate::solutions::Solution;
pub struct DaySix;

impl Solution for DaySix {
    fn solve(&self, lines: Vec<String>) -> () {
        let start_time = std::time::Instant::now();
        let part_one_answer = part_one(lines.clone());
        let elapsed = start_time.elapsed();
        println!("Day 6 part 1 answer: {} ({:?})", part_one_answer, elapsed);
        let start_time = std::time::Instant::now();
        let part_two_answer = part_two(lines);
        let elapsed = start_time.elapsed();
        println!("Day 6 part 2 answer: {} ({:?})", part_two_answer, elapsed);
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

fn part_two(lines: Vec<String>) -> i32 {
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

    let mut total_wins = 0;
    for i in 0..time {
        let current_distance = i * (time - i);
        if current_distance > distance {
            total_wins += 1;
        }
    }

    total_wins
}

#[cfg(test)]
mod tests {
    use crate::utils::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let lines = read_file(&"2023/example6".to_string());
        assert_eq!(part_one(lines), 288);
    }

    #[test]
    fn test_part_two() {
        let lines = read_file(&"2023/example6".to_string());
        assert_eq!(part_two(lines), 71503);
    }
}
