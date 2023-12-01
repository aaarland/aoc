use crate::solutions::AdventSolution;

mod one;
mod two;

pub fn get_solutions() -> Vec<AdventSolution> {
    vec![Box::new(one::DayOne), Box::new(two::DayTwo)]
}
