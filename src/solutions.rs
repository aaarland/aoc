use std::{
    collections::HashMap,
    io::{Stdout, Write, stdout},
    thread::{self, Thread},
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
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
pub type AdventSolution = Box<dyn Solution + Send + Sync>;
pub type UpdateFn<'a> = &'a mut dyn FnMut(Vec<String>);
pub trait Solution {
    fn solve(&self, lines: Vec<String>, part: Part, update: Option<UpdateFn>) -> String;
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

struct Animator {
    stdout: Stdout,
}

impl Animator {
    fn new() -> Self {
        let stdout = stdout();
        Animator { stdout }
    }

    fn animate(&mut self, solution: impl FnOnce(&mut dyn FnMut(Vec<String>)) -> String) -> String {
        self.stdout.execute(EnterAlternateScreen).unwrap();
        self.stdout.execute(Hide).unwrap();
        let mut cb = |current_state: Vec<String>| {
            if let Err(e) = (|| -> Result<(), std::io::Error> {
                self.stdout.execute(Clear(ClearType::All))?;
                self.stdout.execute(MoveTo(0, 0))?;

                for (y, row) in current_state.iter().enumerate() {
                    self.stdout.execute(MoveTo(0, y as u16))?;
                    write!(self.stdout, "{}", row)?;
                }
                thread::sleep(Duration::from_millis(10));
                self.stdout.flush()?;
                Ok(())
            })() {
                eprintln!("Error! {}", e);
            }
        };
        let res = solution(&mut cb);
        self.stdout.execute(Show).unwrap();
        self.stdout.execute(LeaveAlternateScreen).unwrap();
        res
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
        let lines_part_one = lines.clone();
        let mut animator = Animator::new();
        let part_one =
            animator.animate(|update| solution.solve(lines_part_one, Part::One, Some(update)));

        println!(
            "Part 1: {}",
            part_one,
        );
        let part_two =
            animator.animate(|update| solution.solve(lines, Part::Two, Some(update)));
        println!(
            "Part 2: {}",
            part_two,
        );
    }
}
