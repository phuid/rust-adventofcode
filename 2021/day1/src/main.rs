use std::{fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut vec: Vec<i32> = Vec::new();

    for i in contents.lines() {
        vec.push(
            i.trim()
                .parse()
                .expect("please give me correct string number!")
        );
    }

    let mut count = 0;

    for i in 1..vec.len() {
        // println!("{}", vec[i] > vec[i - 1]);
        if vec[i] > vec[i - 1] {
            count += 1;
        }
    }

    println!("part one: {}", count);

    count = 0;

    for i in 3..vec.len() {
        if vec[i - 2] + vec[i - 1] + vec[i] > vec[i - 3] + vec[i - 2] + vec[i - 1] {
            count += 1;
        }
    }

    println!("part two: {}", count);
}
