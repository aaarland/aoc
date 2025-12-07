use crate::solutions::{Part, Solution, UpdateFn};

//Start day 4
//
pub struct DayFour;
impl Solution for DayFour {
    fn solve(&self, lines: Vec<String>, _part: Part, _: Option<UpdateFn>) -> String {
        day_four_part_one(lines).to_string()
    }
}

fn get_nth_char(line: &str, nth: usize) -> u32 {
    line.chars().nth(nth).unwrap().to_digit(10).unwrap()
}

fn day_four_part_one(lines: Vec<String>) -> usize {
    let mut cleaned_rooms = vec![false; 99];
    let mut total = 0;
    for line in lines {
        let mut split_lines = line.split(',');
        let first_half = split_lines.next().unwrap();
        let second_half = split_lines.next().unwrap();
        let mut start = get_nth_char(first_half, 0);
        let mut end = get_nth_char(first_half, 2);

        let room_is_covered = &cleaned_rooms[start as usize..end as usize]
            .iter()
            .all(|room| *room);
        if *room_is_covered {
            total += 1;
        }
        println!("Room is covered : {} ", room_is_covered);

        for index in start..end {
            cleaned_rooms[index as usize] = true;
        }
        start = get_nth_char(second_half, 0);
        end = get_nth_char(second_half, 2);
        let room_is_covered = &cleaned_rooms[start as usize..end as usize]
            .iter()
            .all(|room| *room);
        if *room_is_covered {
            total += 1;
        }
        for index in start..end {
            cleaned_rooms[index as usize] = true;
        }
    }
    return total;
}
