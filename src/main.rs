use core::panic;
use std::env;

use sqlx::{Connection, SqliteConnection};
use utils::{get_pool, read_db};

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
    let pool = get_pool().await;
    let lines = read_db(pool, day, year, config.file);

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
