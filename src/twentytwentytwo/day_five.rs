pub fn day_five(lines: Vec<String>) {
    let new_lines = split_lines(lines);
    let first_part = &new_lines[0];
    let second_part = &new_lines[1];

    let 

}

fn split_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    lines
        .iter()
        .map(|line| line.split("").map(|s| s.to_string()).collect())
        .collect()
}
