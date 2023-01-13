use std::fs;
use std::io::BufRead;
use std::io::BufReader;

use crate::solutions::run_solutions;
mod menu;
mod solutions;
mod twentytwentyone;
mod twentytwentytwo;
fn main() {
    loop {
        let config = menu::Config::new().unwrap();
        let file = fs::File::open(&config.file).expect("No Such File");
        let buf = BufReader::new(file);
        let lines = buf
            .lines()
            .map(|l| l.expect("Could not parse line"))
            .collect();
        println!("Day {} : In file {}", config.day, config.file);

        let advent_funcs_2021 = [
            twentytwentyone::day_one,
            twentytwentyone::day_two,
            twentytwentyone::day_three,
        ];

        match config.year {
            2021 => advent_funcs_2021[config.day - 1](lines),
            2022 => run_solutions(config, lines),
            _ => println!("No such year"),
        }
    }
}
