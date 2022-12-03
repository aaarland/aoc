pub fn day_one(lines: Vec<String>) {
    let mut all_total_calories: Vec<i32> = vec![];
    let mut total_calories = 0;
    for line in lines {
        if line.is_empty() {
            all_total_calories.push(total_calories);
            total_calories = 0;
            continue;
        }
        total_calories += line.parse::<i32>().expect("Failed to parse");
    }
    all_total_calories.sort();
    total_calories = 0;
    let mut index = all_total_calories.len() - 1;

    while index >= all_total_calories.len() - 3 {
        total_calories += all_total_calories[index];
        print!("{} ", index);
        index -= 1;
    }
    println!("The max of calories is {}", total_calories)
}


fn check_letter(letter: Option<&str>){
    match letter {
        Some("X")
        _ => todo!()
    }
}
pub fn day_two(lines: Vec<String>) {
    let all_letters = lines.iter().map(|line| line.split(' '));
    for mut letters in all_letters{
        match letters.next(){
            Some("A") => todo!(),
            Some("B") => todo!(),
            Some("C") => todo!(),
            Some(&_) => todo!(),
            None => panic!()
        }
    }
}
