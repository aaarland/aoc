use std::env;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;

use crate::twentytwentytwo::get_all_funcs;
mod twentytwentyone;
mod twentytwentytwo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let year: &usize = &args[1].parse().expect("Could not parse year");
    let day: &usize = &args[2].parse().expect("could not parse day");
    let file_path = &args[3];
    let file = fs::File::open(file_path).expect("No Such File");
    let buf = BufReader::new(file);
    let lines = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    println!("Day {} : In file {}", day, file_path);

    let advent_funcs_2021 = [
        twentytwentyone::day_one,
        twentytwentyone::day_two,
        twentytwentyone::day_three,
    ];

    let advent_funcs_2022 = get_all_funcs(); 
    match year {
        2021 => advent_funcs_2021[day - 1](lines),
        2022 => advent_funcs_2022[day - 1](lines),
        &_ => ()
    }
}