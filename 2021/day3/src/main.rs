use std::fs;

//https://stackoverflow.com/a/64446974
fn binary_to_u32(s: String) -> u32 {
    let mut binary_digit = s.chars().count();
    let mut real_num: u32 = 0;
    for c in s.chars() {
        let mut temp_var = 2u32.pow(binary_digit.try_into().unwrap());
        temp_var /= 2;
        if c == '1' {
            real_num += temp_var;
        }
        binary_digit -= 1;
    }
    return real_num;
}

fn main() {
    println!("2021 day 3 by phuid");

    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut result: Vec<i32> = Vec::new();

    for _i in 1..input.find("\n").unwrap() {
        result.push(0);
    }

    for line in input.lines() {
        for i in 0..line.len() {
            result[i] += -1 + (2 * (line.chars().nth(i).unwrap() as i32 - '0' as i32));
            // println!("{}", -1 + (2 * (line.chars().nth(i).unwrap() as i32 - '0' as i32)));
        }
    }

    // println!("{:?}", result);

    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();

    for num in result {
        gamma.push_str(&((num > 0) as i32).to_string());
        epsilon.push_str(&((num < 0) as i32).to_string());
    }

    println!(
        "part one: {}",
        binary_to_u32(gamma) * binary_to_u32(epsilon)
    );

    let mut inputvector: Vec<String> = Vec::new();

    for line in input.lines() {
        inputvector.push(line.to_string());
    }

    while inputvector.len() > 1 {
        for bit in 0..inputvector[0].len() {
            result = Vec::new();
            for _i in 0..inputvector[0].len() {
                result.push(0);
            }

            for line in inputvector.iter() {
                for i in 0..line.len() {
                    result[i] += -1 + (2 * (line.chars().nth(i).unwrap() as i32 - '0' as i32));
                    // println!("{}", -1 + (2 * (line.chars().nth(i).unwrap() as i32 - '0' as i32)));
                }
            }
            // println!("{:?}; {:?}", result, inputvector);

            let mut i = 0;
            while i < inputvector.len() {
                // println!("{}: {:?} [{}] = {} || {}; len:{}", i, inputvector[i], bit, inputvector[i].chars().nth(bit).unwrap(), if result[bit] >= 0 { '1' } else { '0' }, inputvector.len());
                if inputvector[i].chars().nth(bit).unwrap() != if result[bit] >= 0 { '1' } else { '0' } {
                    inputvector.remove(i);
                }
                else {
                    i += 1;
                }
            }
            if inputvector.len() == 1 {
                // println!("fin{:?}", inputvector[0]);
                break;
            }
        }
    }

    let ox = inputvector[0].clone();

    inputvector = Vec::new();

    for line in input.lines() {
        inputvector.push(line.to_string());
    }

    while inputvector.len() > 1 {
        for bit in 0..inputvector[0].len() {
            result = Vec::new();
            for _i in 0..inputvector[0].len() {
                result.push(0);
            }

            for line in inputvector.iter() {
                for i in 0..line.len() {
                    result[i] += -1 + (2 * (line.chars().nth(i).unwrap() as i32 - '0' as i32));
                    // println!("{}", -1 + (2 * (line.chars().nth(i).unwrap() as i32 - '0' as i32)));
                }
            }
            // println!("{:?}; {:?}", result, inputvector);

            let mut i = 0;
            while i < inputvector.len() {
                // println!("{}: {:?} [{}] = {} || {}; len:{}", i, inputvector[i], bit, inputvector[i].chars().nth(bit).unwrap(), if result[bit] >= 0 { '1' } else { '0' }, inputvector.len());
                if inputvector[i].chars().nth(bit).unwrap() == if result[bit] >= 0 { '1' } else { '0' } {
                    inputvector.remove(i);
                }
                else {
                    i += 1;
                }
            }
            if inputvector.len() == 1 {
                // println!("finco{:?}", inputvector[0]);
                break;
            }
        }
    }

    let co = inputvector[0].clone();

    println!("part two: {}", binary_to_u32(ox) * binary_to_u32(co));
}
