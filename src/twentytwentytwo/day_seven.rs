fn solution(lines: Vec<String>) {
    //each line is a command
    //split the string by spaces
    for line in lines {
        let mut split_line = line.split(" ");
        let command = split_line.next().unwrap();
        let argument = split_line.next().unwrap();
        //regex for number
        let number = Regex::new(r"[-+]?[0-9]+").unwrap();
        // regex for $
        let dollar = Regex::new(r"\$").unwrap();
        // regex for 'dir'
        let dir = Regex::new(r"dir").unwrap();
        match command {
            number => {
                //do something
                println!("Number");
                println("{} : {}", command, argument);
            }
            dir => {
                //do something
                println!("Dir");
                println("{} : {}", command, argument);
            }
            dollar=> {
                //do something
                println!("Dollar");
                println("{} : {}", command, argument);
                
            }
            _ => {
                //do something
            }
        }
    }
}
