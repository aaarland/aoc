use crate::menu::Config;

use crate::twentytwentytwo::get_all_funcs;

pub struct Solutions(Vec<Solution>);

struct Solution<> {
    solution: fn(Vec<String>) -> (),
}

impl Solutions {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, solution: fn(Vec<String>) -> ()) {
        self.0.push(Solution { solution });
    }

    fn run(&self, day: usize, lines: Vec<String>) {
        (self.0[day - 1].solution)(lines);
    }

}

pub fn run_solutions(config: Config, lines: Vec<String>) {
    let mut solutions = Solutions::new();
    for solution in get_all_funcs() {
        solutions.add(solution);
    }
    solutions.run(config.day, lines);
}