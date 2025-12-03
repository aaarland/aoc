use std::collections::HashMap;

use maplit::hashmap;

use crate::solutions::{AdventSolution, Date};

mod one;
mod two;

pub fn get_solutions() -> HashMap<Date, AdventSolution> {
    hashmap! {
        Date::First => Box::new(one::DayOne) as AdventSolution,
        Date::Second => Box::new(two::DayTwo) as AdventSolution,
    }
}
