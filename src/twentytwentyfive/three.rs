use crate::define_solution;

define_solution!(DayThree, part_one, part_two);

fn part_one(lines: Vec<String>) -> String {
    lines
        .into_iter()
        .fold(0, |acc, next| {
            let mut next_chars = next.chars().rev();
            let first_max = next_chars.by_ref().take(1).next().unwrap();

            let result = next_chars.fold(
                (0, first_max.to_digit(10).unwrap()),
                |mut max_values, next_char| {
                    let current = next_char.to_digit(10).unwrap();
                    if current >= max_values.0 {
                        max_values.1 = max_values.1.max(max_values.0);
                        max_values.0 = current;
                    }
                    max_values
                },
            );
            acc + (result.0 * 10) + result.1
        })
        .to_string()
}

fn part_two(lines: Vec<String>) -> String {
    lines
        .into_iter()
        .fold(0, |acc, next| {
            let mut the_num = [0 as u64; 12];
            let the_num_len = the_num.len();
            let mut start = next.chars().skip(next.len() - the_num_len);
            for i in 0..the_num.len() {
                the_num[i] = start.next().unwrap().to_digit(10).unwrap() as u64;
            }

            find_max(&mut the_num, &next, 0, next.len() - the_num_len);
            dbg!(the_num);
            acc + the_num.into_iter().fold(0 as u64, |inner_acc, d| inner_acc * 10 + d)
            // let mut next_chars = next.chars().rev();
            // let first_max = next_chars.by_ref().take(1).next().unwrap();
            //
            // let result = next_chars.fold(
            //     (0, first_max.to_digit(10).unwrap()),
            //     |mut max_values, next_char| {
            //         let current = next_char.to_digit(10).unwrap();
            //         if current >= max_values.0 {
            //             max_values.1 = max_values.1.max(max_values.0);
            //             max_values.0 = current;
            //         }
            //         max_values
            //     },
            // );
            // acc + (result.0 * 10) + result.1
        })
        .to_string()
}

fn find_max(the_num: &mut [u64; 12], next: &str, current_index: usize, start_index: usize) -> () {
    if current_index >= the_num.len() || start_index == 0{
        return;
    }
    let next_num = next.chars().nth(start_index - 1).unwrap().to_digit(10).unwrap() as u64;
    if the_num[current_index] >= next_num {
        the_num[current_index] = next_num;
        find_max(the_num, next, current_index, start_index - 1);
    }
    find_max(the_num, next, current_index + 1, start_index + 1);
}
