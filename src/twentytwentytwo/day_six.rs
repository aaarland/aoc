use std::collections::HashSet;

pub fn solution(lines: Vec<String>){
    let mut index = 0;
    for _ in lines[0].chars(){
        let mut stack = HashSet::new();
        for n in index..index+14{
            if stack.contains(&lines[0].chars().nth(n).unwrap()){
                break;
            }
            stack.insert(lines[0].chars().nth(n).unwrap());
            // find only unique chars in stack
            if n == index + 13 && stack.len() == 14 {
                println!("{}", index + 14);
                return;
            }
        }
        index += 1;
    }
}