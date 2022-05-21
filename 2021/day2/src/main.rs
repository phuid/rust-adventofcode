use std::{fs};
use substring::Substring;

fn main() {
    println!("2021 day 2 by phuid");

    let inputstring = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let mut inputvector: Vec<&str> = Vec::new();

    for i in inputstring.lines() {
        inputvector.push(i);
    }

    let mut x = 0;
    let mut y = 0;
    
    for i in inputvector.iter() {
        if i.contains("forward") {
            // println!("x: {} += {}", x, i.substring(8, i.len()).parse::<i32>().unwrap());
            x += i.substring(8, i.len()).parse::<i32>().unwrap();
        }
        else if i.contains("down") {
            // println!("y: {} += {}", y, i.substring(5, i.len()).parse::<i32>().unwrap());
            y += i.substring(5, i.len()).parse::<i32>().unwrap();
        }
        else if i.contains("up") {
            // println!("y: {} -= {}", y, i.substring(3, i.len()).parse::<i32>().unwrap());
            y -= i.substring(3, i.len()).parse::<i32>().unwrap();
        }
        else {
            println!("ERROR: {}", i);
        }
    }

    println!("x: {}, y: {}", x, y);
    println!("part one: {}", x.abs() * y.abs());
    
    x = 0;
    y = 0;
    let mut aim = 0;

    for i in inputvector.iter() {
        if i.contains("forward") {
            // println!("x: {} += {}", x, i.substring(8, i.len()).parse::<i32>().unwrap());
            x += i.substring(8, i.len()).parse::<i32>().unwrap();
            y += aim * i.substring(8, i.len()).parse::<i32>().unwrap();
        }
        else if i.contains("down") {
            // println!("y: {} += {}", y, i.substring(5, i.len()).parse::<i32>().unwrap());
            aim += i.substring(5, i.len()).parse::<i32>().unwrap();
        }
        else if i.contains("up") {
            // println!("y: {} -= {}", y, i.substring(3, i.len()).parse::<i32>().unwrap());
            aim -= i.substring(3, i.len()).parse::<i32>().unwrap();
        }
        else {
            println!("ERROR: {}", i);
        }
    }

    println!("x: {}, y: {}", x, y);
    println!("part two: {}", x.abs() * y.abs());
}
