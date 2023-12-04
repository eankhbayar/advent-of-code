use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file_path = "input.in";
    let mut file = File::open(file_path).expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    let mut sum: i32 = 0;
    for line in contents.lines() {
        let mut first: i32 = -1;
        let mut second: i32 = -1;
        for ch in line.chars() {
            if ch >= '0' && ch <= '9' {
                if first == -1 {
                    first = ch.to_digit(10).unwrap() as i32;
                } else {
                    second = ch.to_digit(10).unwrap() as i32;
                }
            }
        }
        if second == -1i32 {
            second = first;
        }
        sum += first * 10 + second;
    }
    
    println!("Sum: {}", sum);

}
