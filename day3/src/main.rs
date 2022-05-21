use std::{fs};

//https://stackoverflow.com/a/64446974
fn binary_to_u32(s: String) -> u32 {
    let mut binary_digit =  s.chars().count();
    let mut real_num: u32 = 0;
    for c in s.chars() { 
        let mut temp_var = 2u32.pow(binary_digit.try_into().unwrap());
        temp_var /= 2;
        if c == '1'{
            real_num += temp_var;
        }
        binary_digit -= 1;
    }
    return real_num; }
    

fn main() {
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

    println!("part one: {}", binary_to_u32(gamma) * binary_to_u32(epsilon));
}
