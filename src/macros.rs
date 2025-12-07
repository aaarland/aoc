#[macro_export]
macro_rules! define_solution {
    ($name:ident, $solve_one:expr, $solve_two:expr) => {
        pub struct $name;

        impl crate::solutions::Solution for $name {
            fn solve(&self, lines: Vec<String>, part: crate::solutions::Part, update: Option<&mut dyn FnMut(String)>) -> String {
                match part {
                    crate::solutions::Part::One => $solve_one(lines),
                    crate::solutions::Part::Two => $solve_two(lines),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_solution_with_callback {
    ($name:ident, $solve_one:ident, $solve_two:ident) => {
        pub struct $name;

        impl crate::solutions::Solution for $name {
            fn solve(
                &self,
                lines: Vec<String>,
                part: crate::solutions::Part,
                update: Option<&mut dyn FnMut(String)>,
            ) -> String {
                match part {
                    crate::solutions::Part::One => $solve_one(lines, update),
                    crate::solutions::Part::Two => $solve_two(lines, update),
                }
            }
        }
    };
}
