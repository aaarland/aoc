mod day_five;
mod day_four;
mod day_one;
mod day_six;
mod day_three;
mod day_two;
mod day_seven;

pub fn get_all_funcs() -> [fn(Vec<String>); 7] {
    return [
        day_one::solution,
        day_two::solution,
        day_three::solution,
        day_four::solution,
        day_five::solution,
        day_six::solution,
        day_seven::solution,
    ];
}
