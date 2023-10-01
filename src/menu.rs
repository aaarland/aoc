use std::io;


pub struct Config {
    pub year: usize,
    pub day: usize,
    pub file: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let year;
        let day;
        let file;
        if let Some(arg) = args.next() {
            year = arg.parse::<usize>().expect("Please type a number!");
        }else{
            year = get_years();
        }
        if let Some(arg) = args.next() {
            day = arg.parse::<usize>().expect("Please type a number!");
        }else {
            day = get_days();
        }

        if let Some(arg) = args.next() {
            file = arg;
        }else {
            file = get_file();
        }
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




