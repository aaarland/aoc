use std::fmt::{Display};

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub year: Option<usize>,
    #[arg(short, long)]
    pub day: Option<usize>,
    #[arg(value_enum)]
    pub input: Option<Input>,
    #[arg(short, long)]
    pub frame_rate: Option<usize>,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Input {
    Example,
    Day,
}
impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Input::Example => "example",
            Input::Day => "day",
        };
        write!(f, "{}", res)
    }
}
