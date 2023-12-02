use crate::solutions::AdventSolution;

mod eight;
mod eighteen;
mod eleven;
mod fifteen;
mod five;
mod four;
mod fourteen;
mod nine;
mod nineteen;
mod one;
mod seven;
mod seventeen;
mod six;
mod sixteen;
mod ten;
mod thirteen;
mod three;
mod twelve;
mod twenty;
mod twentyfive;
mod twentyfour;
mod twentyone;
mod twentythree;
mod twentytwo;
mod two;

pub fn get_solutions() -> Vec<AdventSolution> {
    vec![
        Box::new(one::DayOne),
        Box::new(two::DayTwo),
        Box::new(three::DayThree),
        Box::new(four::DayFour),
        Box::new(five::DayFive),
        Box::new(six::DaySix),
        Box::new(seven::DaySeven),
        Box::new(eight::DayEight),
        Box::new(nine::DayNine),
        Box::new(ten::DayTen),
        Box::new(eleven::DayEleven),
        Box::new(twelve::DayTwelve),
        Box::new(thirteen::DayThirteen),
        Box::new(fourteen::DayFourteen),
        Box::new(fifteen::DayFifteen),
        Box::new(sixteen::DaySixteen),
        Box::new(seventeen::DaySeventeen),
        Box::new(eighteen::DayEighteen),
        Box::new(nineteen::DayNineteen),
        Box::new(twenty::DayTwenty),
        Box::new(twentyone::DayTwentyOne),
        Box::new(twentytwo::DayTwentyTwo),
        Box::new(twentythree::DayTwentyThree),
        Box::new(twentyfour::DayTwentyFour),
        Box::new(twentyfive::DayTwentyFive),
    ]
}
