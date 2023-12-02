use std::{io::{BufReader, BufRead}, fs::File};

pub fn read_file(location: &String) -> Vec<String> {
    let file = File::open(location).expect("Could not open file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
