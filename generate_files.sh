#!/bin/bash

YEAR=$1

DAY_AS_STRING=('one' 'two' 'three' 'four' 'five' 'six' 'seven' 'eight' 'nine' 'ten' 'eleven' 'twelve' 'thirteen' 'fourteen' 'fifteen' 'sixteen' 'seventeen' 'eighteen' 'nineteen' 'twenty' 'twentyone' 'twentytwo' 'twentythree' 'twentyfour' 'twentyfive')

function template() {
    DAY=$1
    echo """
use crate::define_solution;

define_solution!(DayTwo, part_one, part_two);
fn part_one(lines: Vec<String>) -> String {
    String::new()
}

fn part_two(lines: Vec<String>) -> String {
    String::new()
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
