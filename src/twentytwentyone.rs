pub fn day_one(lines: Vec<String>) {
    let msg = "Expected a number";
    let mut prev_line: usize = lines[0].parse::<usize>().expect(msg)
        + lines[1].parse::<usize>().expect(msg)
        + lines[2].parse::<usize>().expect(msg);
    let mut increase = 0;
    for i in 1..lines.len() - 2 {
        let curr_line = lines[i].parse::<usize>().expect(msg)
            + lines[i + 1].parse::<usize>().expect(msg)
            + lines[i + 2].parse::<usize>().expect(msg);
        if curr_line > prev_line {
            increase += 1;
        }
        prev_line = curr_line;
    }
    println!("Increased {} times", increase);
}

pub fn day_two(lines: Vec<String>) {
    let mut x = 0;
    let mut aim = 0;
    let mut depth = 0;
    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let tempnum = split[1].parse::<i64>().expect("expected a number");
        match split[0] {
            "forward" => {
                x += tempnum;
                depth += tempnum * aim;
            }
            "down" => aim += tempnum,
            "up" => aim -= tempnum,
            &_ => todo!(),
        }
    }
    println!("{} {} {}", x, depth, x * depth)
}

pub fn day_three_rec(
    index: usize,
    lines: Vec<String>,
    zero: &mut [i32; 12],
    ones: &mut [i32; 12],
    low: u32,
    high: u32,
) -> String {
    if index >= 12 || lines.len() == 1 {
        return lines.first().expect("help").clone();
    }
    for line in &lines {
        if line.chars().nth(index).expect("help").to_digit(10) == Some(low) {
            zero[index] += 1;
        } else {
            ones[index] += 1;
        }
    }
    let new_lines;
    println!("{}", zero[index] <= ones[index]);
    if zero[index] <= ones[index] {
        new_lines = lines
            .into_iter()
            .filter(|l| l.chars().nth(index).expect("help").to_digit(10) == Some(low))
            .collect::<Vec<String>>();
    } else {
        new_lines = lines
            .into_iter()
            .filter(|l| l.chars().nth(index).expect("help").to_digit(10) == Some(high))
            .collect::<Vec<String>>();
    }
    // println!("{} {} {} {} ", zero[i], ones[i], epsilon, gamma);

    return day_three_rec(index + 1, new_lines, zero, ones, low, high);
}

pub fn day_three(lines: Vec<String>) {
    let mut zero_oxygen = [0; 12];
    let mut one_oxygen = [0; 12];
    let mut zero_co2 = [0; 12];
    let mut one_co2 = [0; 12];
    let result_oxygen = day_three_rec(0, lines.clone(), &mut zero_oxygen, &mut one_oxygen, 0, 1);
    let result_co2 = day_three_rec(0, lines, &mut zero_co2, &mut one_co2, 0, 1);
    let mut oxygen = 0;
    let mut co2 = 0;

    println!("{} {}", result_co2, result_oxygen);
    for i in 0..5 {
        oxygen <<= 1;
        co2 <<= 1;
        oxygen |= result_oxygen
            .chars()
            .nth(i)
            .expect("help")
            .to_digit(10)
            .expect("help");
        co2 |= result_co2
            .chars()
            .nth(i)
            .expect("help")
            .to_digit(10)
            .expect("help");
    }

    println!("{:#012b} {:#012b}", co2, oxygen);
    println!("{}", co2 * oxygen);

    return;
    // let mut zero = [0; 12];
    // let mut ones = [0; 12];
    // let mut epsilon = 1;
    // let mut gamma = 1;
    // for line in &lines {
    //     let mut index = 0;
    //     for c in line.chars() {
    //         if c.to_digit(10) == Some(0) {
    //             zero[index] += 1;
    //         } else {
    //             ones[index] += 1;
    //         }
    //         index += 1;
    //     }
    // }
    // for i in 0..12 {
    //     if zero[i] <= ones[i] {
    //         epsilon <<= 1;
    //         gamma <<= 1;
    //         gamma |= 1;
    //     } else {
    //         epsilon <<= 1;
    //         epsilon |= 1;
    //         gamma <<= 1;
    //     }
    //     println!("{} {} {} {} ", zero[i], ones[i], epsilon, gamma);
    // }
    // println!("{:?} {:?}", zero, ones)
    // println!("{}", (epsilon & 4095) * (gamma & 4095))
}
