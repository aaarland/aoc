use std::collections::HashMap;

use maplit::hashmap;

use crate::solutions::{AdventSolution, Date};

mod day_five;
mod day_four;
mod day_one;
mod day_seven;
mod day_six;
mod day_three;
mod day_two;

pub fn get_solutions() -> HashMap<Date, AdventSolution> {
    hashmap! {
        Date::First => Box::new(day_one::DayOne) as AdventSolution,
        Date::Second => Box::new(day_two::DayTwo) as AdventSolution,
        Date::Third => Box::new(day_three::DayThree),
        Date::Forth => Box::new(day_four::DayFour),
        Date::Fifth => Box::new(day_five::DayFive),
        Date::Sixth => Box::new(day_six::DaySix),
        Date::Seventh => Box::new(day_seven::DaySeven),
    }
}
