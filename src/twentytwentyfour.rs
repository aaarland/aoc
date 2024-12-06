use std::collections::HashMap;

use maplit::hashmap;

use crate::solutions::{AdventSolution, Date};

mod four;
mod one;
mod three;
mod two;

pub fn get_solutions() -> HashMap<Date, AdventSolution> {
    hashmap! {
        Date::First => Box::new(one::DayOne) as AdventSolution,
        Date::Second => Box::new(two::DayTwo) as AdventSolution,
        Date::Third => Box::new(three::DayThree) as AdventSolution,
        Date::Fourth=> Box::new(four::DayFour) as AdventSolution
    }
}
