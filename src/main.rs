use std::env;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

use crate::solutions::AdventCalendarYear;

mod menu;
mod solutions;
mod twentytwentyone;
mod twentytwentytwo;
mod twentytwentythree;

fn main() {
    loop {
        let config = menu::Config::new(env::args()).unwrap();
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
            2022 => AdventCalendarYear::TwentyTwentyTwo.run(config.day, lines),
            2023 => AdventCalendarYear::TwentyTwentyThree.run(config.day, lines),
            _ => println!("No such year"),
        }

        let mut end_program = String::new();
        println!("Would you like to end the program? (y/n)");
        io::stdin()
            .read_line(&mut end_program)
            .expect("Failed to read line");
        let end_program = end_program.trim();

        match end_program {
            "y" => break,
            "n" => continue,
            _ => println!("Please type y or n"),
        }
    }
}
