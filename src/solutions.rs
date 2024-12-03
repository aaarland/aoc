
pub enum Part {
    One,
    Two,
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
    fn get_solutions(&self) -> Vec<AdventSolution> {
        match self {
            AdventCalendarYear::TwentyTwentyTwo => crate::twentytwentytwo::get_solutions(),
            AdventCalendarYear::TwentyTwentyThree => crate::twentytwentythree::get_solutions(),
            AdventCalendarYear::TwentyTwentyFour => crate::twentytwentyfour::get_solutions(),
        }
    }
    pub fn run(&self, day: usize, lines: Vec<String>) {
        let solutions = self.get_solutions();
        let part_one = (solutions[day - 1]).solve(lines.clone(), Part::One);
        let part_two = (solutions[day - 1]).solve(lines, Part::Two);
        println!("Part 1: {part_one}");
        println!("Part 1: {part_two}");
    }
}
