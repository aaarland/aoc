use crate::solutions::AdventSolution;

mod one;

pub fn get_solutions() -> Vec<AdventSolution> {
    vec![
        Box::new(one::DayOne),
    ]
}
