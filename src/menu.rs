use std::io;


pub struct Config {
    pub year: usize,
    pub day: usize,
    pub file: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let year = get_years();
        let day = get_days();
        let file = get_file();
        Ok(Config { year, day, file })
    }
}

fn get_years() -> usize{
    let years = vec![2021, 2022];
    let mut year = String::new();
    println!("Which year would you like to run?");
    for year in years {
        println!("{}", year);
    }
    io::stdin()
        .read_line(&mut year)
        .expect("Failed to read line");
    let year: usize = year.trim().parse().expect("Please type a number!");
    year
}

fn get_days() -> usize{
    let days = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
    let mut day = String::new();
    println!("Which day would you like to run?");
    for day in days {
        println!("{}", day);
    }
    io::stdin()
        .read_line(&mut day)
        .expect("Failed to read line");
    let day: usize = day.trim().parse().expect("Please type a number!");
    day
}

fn get_file() -> String{
    let mut file = String::new();
    println!("Which file would you like to run?");
    io::stdin()
        .read_line(&mut file)
        .expect("Failed to read line");
    let file: String = file.trim().parse().expect("Please type a number!");
    file
}




