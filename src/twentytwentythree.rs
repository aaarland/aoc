use std::collections::HashMap;

use maplit::hashmap;

use crate::solutions::{AdventSolution, Date};

mod eight;
mod eighteen;
mod eleven;
mod five;
mod four;
mod one;
mod seven;
mod six;
mod sixteen;
mod thirteen;
mod three;
mod two;

pub fn get_solutions() -> HashMap<Date, AdventSolution> {
    hashmap!{
        Date::First => Box::new(one::DayOne) as AdventSolution,
        Date::First => Box::new(two::DayTwo),
        Date::First => Box::new(three::DayThree),
        Date::First => Box::new(four::DayFour),
        Date::First => Box::new(five::DayFive),
        Date::First => Box::new(six::DaySix),
        Date::First => Box::new(seven::DaySeven),
        Date::First => Box::new(eight::DayEight),
        Date::Eleventh => Box::new(eleven::DayEleven),
        Date::ThirTeen => Box::new(thirteen::DayThirteen),
        Date::SixTeen => Box::new(sixteen::DaySixteen),
        Date::EightTeen => Box::new(eighteen::DayEighteen),
    }
}
