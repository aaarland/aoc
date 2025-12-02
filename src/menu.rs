use std::io;
use std::process::exit;
use std::time::Duration;

use crossterm::cursor;
use crossterm::event::poll;
use crossterm::event::read;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::enable_raw_mode;
use crossterm::ExecutableCommand;

pub struct Config {
    pub year: usize,
    pub day: usize,
    pub file: String,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> io::Result<Config> {
        args.next();
        let year;
        let day;
        let file;
        if let Some(arg) = args.next() {
            year = arg.parse::<usize>().expect("Please type a number!");
        } else {
            year = get_years();
        }
        if let Some(arg) = args.next() {
            day = arg.parse::<usize>().expect("Please type a number!");
        } else {
            day = get_days();
        }

        if let Some(arg) = args.next() {
            file = arg;
        } else {
            file = get_file(year, day);
        }
        Ok(Config { year, day, file })
    }
}

fn read_key(height: usize) -> io::Result<usize> {
    let mut stdout = io::stdout();
    let index;
    enable_raw_mode()?;
    stdout.execute(cursor::SavePosition)?;
    let (_, row) = cursor::position()?;
    loop {
        if poll(Duration::from_millis(500))? {
            let (_, current_row) = cursor::position()?;
            match read()? {
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => exit(0),
                    KeyCode::Char('j') => {
                        stdout.execute(cursor::MoveDown(1))?;
                    }
                    KeyCode::Char('k') => {
                        if (row - height as u16 + 1) < current_row {
                            stdout.execute(cursor::MoveUp(1))?;
                        }
                    }
                    KeyCode::Enter => {
                        index = row - current_row;
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }
    stdout.execute(cursor::RestorePosition)?;
    disable_raw_mode()?;
    Ok(height - index as usize - 1)
}
fn get_years() -> usize {
    let years = vec![2021, 2022, 2023, 2024, 2025];
    println!("Which year would you like to run?");
    years
        .iter()
        .take(years.len() - 1)
        .for_each(|year| println!("{}", year));
    print!("{}", years[years.len() - 1]);
    let index = read_key(years.len()).unwrap();
    years[index]
}

fn get_days() -> usize {
    let mut days = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ];
    days.reverse();
    println!("Which day would you like to run?");
    days.iter()
        .take(days.len() - 1)
        .for_each(|day| println!("{}", day));
    print!("{}", days[days.len() - 1]);
    let index = read_key(days.len()).unwrap();
    days[index]
}

fn get_file(year: usize, day: usize) -> String {
    println!("Which file would you like to run?");
    println!("example");
    print!("day");
    let index = read_key(2).unwrap();
    match index {
        0 => format!("{}/example{}", year, day),
        1 => format!("{}/day{}", year, day),
        _ => exit(1),
    }
}
