pub fn day_five(lines: Vec<String>) {
    let (mut i, mut quantity, mut from, mut to): (usize, usize, usize, usize);
    let mut index = 0;
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        matrix.push(vec![]);
    }
    for line in lines {
        if index < 8 {
            i = 0;
            for j in (1..35).step_by(4) {
                if line.chars().nth(j).unwrap() != ' ' {
                    matrix[i].insert(0, line.chars().nth(j).unwrap());
                }
                i += 1;
            }
            index += 1;
        }
        let v = line.split(' ').collect::<Vec<&str>>();
        if v[0] == "move"{
            quantity = v[1].parse().unwrap();
            from = v[3].parse().unwrap();
            to = v[5].parse().unwrap();
            // for _ in 0..quantity{
            //     let temp = matrix[from - 1].pop().unwrap();
            //     matrix[to - 1].push(temp);
            // }
            let fl = matrix[from - 1].len();
            let mut u: Vec<_> = matrix[from - 1].drain((fl - quantity)..).collect();
            matrix[to - 1].append(&mut u);
        }
    }
    for m in matrix {
        println!("{:?}", m[m.len() - 1]);
    }

}

