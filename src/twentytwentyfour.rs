use std::collections::HashMap;

use maplit::hashmap;

use crate::solutions::{AdventSolution, Date};

mod one;

pub fn get_solutions() -> HashMap<Date, AdventSolution> {
    hashmap!{
        Date::First => Box::new(one::DayOne) as AdventSolution
    }
}
