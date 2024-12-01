pub type AdventSolution = Box<dyn Solution>;
pub trait Solution {
    fn solve(&self, lines: Vec<String>) -> ();
}
pub struct AdventCalendarSolutions {
    pub solutions: Vec<AdventSolution>,
}

impl AdventCalendarSolutions {
    fn run(&self, day: usize, lines: Vec<String>) {
        (&self.solutions[day - 1]).solve(lines);
    }
}

pub enum AdventCalendarYear {
    TwentyTwentyTwo,
    TwentyTwentyThree,
    TwentyTwentyFour,
}

impl AdventCalendarYear {
    pub fn run(&self, day: usize, lines: Vec<String>) {
        match self {
            AdventCalendarYear::TwentyTwentyTwo => {
                let calendar = AdventCalendarSolutions {
                    solutions: crate::twentytwentytwo::get_solutions(),
                };
                calendar.run(day, lines);
            }
            AdventCalendarYear::TwentyTwentyThree => {
                let calendar = AdventCalendarSolutions {
                    solutions: crate::twentytwentythree::get_solutions()
                };
                calendar.run(day, lines);
            },
            AdventCalendarYear::TwentyTwentyFour => {
                let calendar = AdventCalendarSolutions {
                    solutions: crate::twentytwentyfour::get_solutions()
                };
                calendar.run(day, lines);

            }
        }
    }
}
