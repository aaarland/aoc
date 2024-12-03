#[macro_export]
macro_rules! define_solution {
    ($name:ident, $solve_one:expr, $solve_two:expr) => {
        pub struct $name;

        impl crate::solutions::Solution for $name {
            fn solve(&self, lines: Vec<String>, part: crate::solutions::Part) -> String {
                match part {
                    crate::solutions::Part::One => $solve_one(lines),
                    crate::solutions::Part::Two => $solve_two(lines),
                }
            }
        }
    };
}
