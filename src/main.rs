use std::env;

use crate::solutions::AdventCalendarYear;

mod macros;
mod menu;
mod solutions;
mod sprites;
mod twentytwentyfour;
mod twentytwentyone;
mod twentytwentythree;
mod twentytwentytwo;
mod utils;

fn main() {
    let config = menu::Config::new(env::args()).unwrap();
    let lines = utils::read_file(&config.file);
    println!("Day {} : In file {}", config.day, config.file);

    let advent_funcs_2021 = [
        twentytwentyone::day_one,
        twentytwentyone::day_two,
        twentytwentyone::day_three,
    ];

    match config.year {
        2021 => advent_funcs_2021[config.day - 1](lines),
        2022 => AdventCalendarYear::TwentyTwentyTwo.run(config.day, lines),
        2023 => AdventCalendarYear::TwentyTwentyThree.run(config.day, lines),
        2024 => AdventCalendarYear::TwentyTwentyFour.run(config.day, lines),
        _ => println!("No such year"),
    }
}
