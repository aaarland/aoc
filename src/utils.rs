use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tokio::sync::OnceCell;

// Use OnceCell for one-time initialization of the connection pool
static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

// Function to initialize the pool and return a reference to it
pub async fn get_pool() -> &'static SqlitePool {
    POOL.get_or_init(|| async {
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite:advent_data.db")
            .await
            .expect("Failed to connect to db")
    })
    .await
}

pub fn read_file(location: &String) -> Vec<String> {
    let file = File::open(location).expect("Could not open file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn extract_day_year(file: &str) -> (i32, i32) {
    let re = Regex::new(r"^(\d{4})/(?:day|example)(\d+)$").unwrap();

    let Some(captures) = re.captures(file) else {
        panic!("Could not parse string {file}");
    };
    let year = captures.get(1).unwrap().as_str();
    let day = captures.get(2).unwrap().as_str();
    (
        day.parse().expect("Failed to parse day {file}"),
        year.parse().expect("Failed to parse year {year}"),
    )
}

pub async fn read_db(
    connection: &sqlx::SqlitePool,
    day: i32,
    year: i32,
    file: String,
) -> Vec<String> {
    let new_lines = sqlx::query!(
        "SELECT example, full FROM aoc where day = ? and year = ?",
        day,
        year
    )
    .fetch_all(connection)
    .await
    .expect("Failed to query lol");
    new_lines
        .iter()
        .map(|thing| match file {
            _ if file.contains("example") => thing.example.clone().unwrap(),
            _ if file.contains("day") => thing.full.clone().unwrap(),
            _ => panic!("couldn't find example or day in file name"),
        })
        .flat_map(|result| {
            result
                .split('\n')
                .map(|string| string.to_string())
                .collect::<Vec<_>>()
        })
        .collect()
}
