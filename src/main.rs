use std::env;

use utils::read_db;
use clap::{Parser};

use crate::{cli::Cli, solutions::AdventCalendarYear};

mod animator;
mod macros;
mod menu;
mod solutions;
mod sprites;
mod twentytwentyfive;
mod twentytwentyfour;
mod twentytwentyone;
mod twentytwentythree;
mod twentytwentytwo;
mod utils;
mod cli;

fn main() {
    let cli = Cli::parse();
    let config = menu::Config::new(cli).unwrap();
    let day = config.day as i32;
    let year = config.year as i32;
    let lines = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async { read_db(day, year, &config.file).await });

    //let lines = utils::read_file(&config.file);
    println!("Day {} : In file {}", config.day, config.file);

    let advent_funcs_2021 = [
        twentytwentyone::day_one,
        twentytwentyone::day_two,
        twentytwentyone::day_three,
    ];

    match config.year {
        2021 => advent_funcs_2021[config.day - 1](lines),
        _ => {
            if let Ok(year) = AdventCalendarYear::try_from(config.year) {
                year.run(config.day, lines)
            } else {
                println!("No such year");
            }
        }
    }
}
