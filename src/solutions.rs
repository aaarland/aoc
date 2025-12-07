use std::{collections::HashMap, time::Instant};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub enum Part {
    One,
    Two,
}

#[derive(Hash, PartialEq, Eq, FromPrimitive)]
pub enum Date {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eight,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
    ThirTeen,
    FourTeen,
    FifTeen,
    SixTeen,
    SevenTeen,
    EightTeen,
    NineTeen,
    Twentieth,
    TwentyFirst,
    TwentySecond,
    TwentyThird,
    TwentyFourth,
    TwentyFifth,
}
pub type AdventSolution = Box<dyn Solution>;
pub trait Solution {
    fn solve(&self, lines: Vec<String>, part: Part) -> String;
}

pub enum AdventCalendarYear {
    TwentyTwentyTwo,
    TwentyTwentyThree,
    TwentyTwentyFour,
    TwentyTwentyFive,
}
impl TryFrom<usize> for AdventCalendarYear {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            2022 => Ok(AdventCalendarYear::TwentyTwentyTwo),
            2023 => Ok(AdventCalendarYear::TwentyTwentyThree),
            2024 => Ok(AdventCalendarYear::TwentyTwentyFour),
            2025 => Ok(AdventCalendarYear::TwentyTwentyFive),
            _ => Err("Outside of range"),
        }
    }
}

impl AdventCalendarYear {
    fn get_solutions(&self) -> HashMap<Date, AdventSolution> {
        match self {
            AdventCalendarYear::TwentyTwentyTwo => crate::twentytwentytwo::get_solutions(),
            AdventCalendarYear::TwentyTwentyThree => crate::twentytwentythree::get_solutions(),
            AdventCalendarYear::TwentyTwentyFour => crate::twentytwentyfour::get_solutions(),
            AdventCalendarYear::TwentyTwentyFive => crate::twentytwentyfive::get_solutions(),
        }
    }
    pub fn run(&self, day: usize, lines: Vec<String>) {
        let solutions = self.get_solutions();
        let Some(current_date) = Date::from_usize(day - 1) else {
            panic!("Date out of range {day}")
        };
        let Some(solution) = solutions.get(&current_date) else {
            panic!("Day not implemented {day}")
        };
        let now = Instant::now();
        let part_one = solution.solve(lines.clone(), Part::One);
        let elapsed_time = now.elapsed();
        println!("Part 1: {} took {} seconds", part_one, elapsed_time.as_secs_f64());
        let now = Instant::now();
        let part_two = solution.solve(lines, Part::Two);
        let elapsed_time = now.elapsed();
        println!("Part 2: {} took {} seconds", part_two, elapsed_time.as_secs_f64());
    }
}
