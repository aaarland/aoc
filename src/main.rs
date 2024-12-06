use core::panic;
use std::env;

use sqlx::{Connection, SqliteConnection};

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

#[tokio::main]
async fn main() {
    let mut connection = SqliteConnection::connect("sqlite:advent_data.db")
        .await
        .expect("failed to connect to db lol");

    let config = menu::Config::new(env::args()).unwrap();
    let day = config.day as i32;
    let year = config.year as i32;
    let new_lines = sqlx::query!(
        "SELECT example, full FROM aoc where day = ? and year = ?",
        day,
        year
    )
    .fetch_all(&mut connection)
    .await
    .expect("Failed to query lol");
    let lines = new_lines
        .iter()
        .map(|thing| match config.file {
            _ if config.file.contains("example") => thing.example.clone().unwrap(),
            _ if config.file.contains("day") => thing.full.clone().unwrap(),
            _ => panic!("couldn't find example or day in file name"),
        })
        .flat_map(|result| {
            result
                .split('\n')
                .map(|string| string.to_string())
                .collect::<Vec<_>>()
        })
        .collect();
    //let lines = utils::read_file(&config.file);
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
