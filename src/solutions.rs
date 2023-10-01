pub type AdventSolution = Box<dyn Solution>;
pub trait Solution {
    fn solve(&self, lines: Vec<String>) -> ();
}

