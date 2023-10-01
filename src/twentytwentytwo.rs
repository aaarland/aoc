use crate::solutions::AdventSolution;

mod day_five;
mod day_four;
mod day_one;
mod day_seven;
mod day_six;
mod day_three;
mod day_two;

pub struct TwentyTwentyTwo {
    pub solutions: Vec<AdventSolution>,
}

impl TwentyTwentyTwo {
    pub fn new() -> Self {
        Self {
            solutions: vec![
                Box::new(day_one::DayOne),
                Box::new(day_two::DayTwo),
                Box::new(day_three::DayThree),
                Box::new(day_four::DayFour),
                Box::new(day_five::DayFive),
                Box::new(day_six::DaySix),
                Box::new(day_seven::DaySeven),
            ],
        }
    }

    pub fn run(&self, day: usize, lines: Vec<String>) {
        (self.solutions[day - 1]).solve(lines);
    }
}
