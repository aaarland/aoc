//Start day 1
pub fn day_one(lines: Vec<String>) {
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
    println!("The max of calories is {}", total_calories)
}
//End day 1

//Start day 2
fn check_letter(letter: Option<&str>, opponent: &str) -> i32 {
    match letter {
        Some("X") => return check_win("X", opponent), //win
        Some("Y") => return check_win("Y", opponent), //tie
        Some("Z") => return check_win("Z", opponent), //loose
        _ => todo!(),
    }
}

fn check_win(letter: &str, opponent: &str) -> i32 {
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let win = 6;
    let tie = 3;
    let loose = 0;
    match letter {
        "Z" => match opponent {
            //winning
            "A" => return paper + win,
            "B" => return scissors + win,
            "C" => return rock + win,
            _ => panic!("This shouldn't happen"),
        },
        "Y" => match opponent {
            //tie
            "A" => return rock + tie,
            "B" => return paper + tie,
            "C" => return scissors + tie,
            _ => panic!("This shouldn't happen"),
        },
        "X" => match opponent {
            //loose
            "A" => return scissors + loose,
            "B" => return rock + loose,
            "C" => return paper + loose,
            _ => panic!("This shouldn't happen"),
        },
        _ => panic!("Something went wrong! {:?}", letter),
    }
}

pub fn day_two(lines: Vec<String>) {
    let all_letters = lines.iter().map(|line| line.split(' '));
    let mut total = 0;
    for mut letters in all_letters {
        total += match letters.next() {
            Some("A") => check_letter(letters.next(), "A"),
            Some("B") => check_letter(letters.next(), "B"),
            Some("C") => check_letter(letters.next(), "C"),
            Some(&_) => todo!(),
            None => panic!(),
        }
    }
    println!("The total is {}", total);
}

//End day 2

//Start day 3
// fn first_part(lines: Vec<String>) -> u32 {
//     let mut total = 0;
//     for character in second_half {
//         if first_half.contains(&character) {
//             if character >= 'A' && character <= 'Z' {
//                 total += character as u32 - 'A' as u32 + 27;
//             }
//             if character >= 'a' && character <= 'z' {
//                 total += character as u32 - 'a' as u32 + 1;
//             }
//             println!("The letter is {}", character);
//             break;
//         }
//     }
//     return total;
// }
pub fn day_three(lines: Vec<String>) {
    //split string into two
    let mut total = 0;
    let mut index = 0;
    while index < lines.len() - 2 {
        for character in lines.get(index).unwrap().chars() {
            if lines.get(index + 1).unwrap().contains(character)
                && lines.get(index + 2).unwrap().contains(character)
            {
                if character >= 'A' && character <= 'Z' {
                    total += character as u32 - 'A' as u32 + 27;
                }
                if character >= 'a' && character <= 'z' {
                    total += character as u32 - 'a' as u32 + 1;
                }
                break;
            }
        }
        index += 3;
    }

    println!("Result is : {} ", total)

    //put the first in a set
    //loop through the other string and
}

//End day 3
//Start day 4

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

pub fn day_four(lines: Vec<String>) {
    println!("Result is : {:?}", day_four_part_one(lines))
}
