use std::collections::HashMap;

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
    Forth,
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
}

impl AdventCalendarYear {
    fn get_solutions(&self) -> HashMap<Date, AdventSolution> {
        match self {
            AdventCalendarYear::TwentyTwentyTwo => crate::twentytwentytwo::get_solutions(),
            AdventCalendarYear::TwentyTwentyThree => crate::twentytwentythree::get_solutions(),
            AdventCalendarYear::TwentyTwentyFour => crate::twentytwentyfour::get_solutions(),
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
        let part_one = solution.solve(lines.clone(), Part::One);
        let part_two = solution.solve(lines, Part::Two);
        println!("Part 1: {part_one}");
        println!("Part 2: {part_two}");
    }
}
