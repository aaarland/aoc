#!/bin/bash

YEAR=$1

DAY_AS_STRING=('one' 'two' 'three' 'four' 'five' 'six' 'seven' 'eight' 'nine' 'ten' 'eleven' 'twelve' 'thirteen' 'fourteen' 'fifteen' 'sixteen' 'seventeen' 'eighteen' 'nineteen' 'twenty' 'twentyone' 'twentytwo' 'twentythree' 'twentyfour' 'twentyfive')

function template() {
    DAY=$1
    echo """
use crate::Solve;
pub struct Day${DAY} {

}

impl Solve for Day${DAY} {
    fn solve(&self, lines: Vec<String>) -> () {
    }

}

fn part_one() {

}

fn part_two() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {

    }

    #[test]
    fn test_part_two() {

    }
}
"""
}
# loop over 1-25 days
for i in {1..25}
do
    mkdir -p "src/${YEAR}"
    # check if a file exists
    if [ ! -f "src/${YEAR}/${DAY_AS_STRING[$i]}.rs" ]; then
        # create a file
        touch "src/${YEAR}/${DAY_AS_STRING[$i]}.rs"
        echo "$(template ${DAY_AS_STRING[$i]^})" > src/${YEAR}/${DAY_AS_STRING[$i]}.rs
    fi
done
