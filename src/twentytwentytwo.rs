
mod day_four;
mod day_one;
mod day_three;
mod day_two;
mod day_five;
mod day_six;

pub fn get_all_funcs() -> [fn(Vec<String>); 6]{
    return [day_one::day_one, day_two::day_two, day_three::day_three, day_four::day_four, day_five::day_five, day_six::day_six]
}
