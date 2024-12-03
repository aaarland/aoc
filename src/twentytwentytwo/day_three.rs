
use crate::solutions::{Part, Solution};

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
pub struct DayThree;
impl Solution for DayThree {
    fn solve(&self, lines: Vec<String>, part: Part) -> String {
        solution(lines)
    }
}
pub fn solution(lines: Vec<String>) -> String {
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


    //put the first in a set
    //loop through the other string and
    total.to_string()
}

//End day 3
