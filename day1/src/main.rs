use std::fs::File;
use std::io::{BufRead, BufReader};

fn _open_file_and_return_lines(filepath: String) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines // Return the lines vector
}

fn _return_first_digit_and_last_digit_from_string(string: &String) -> i32 {
    let spelled_out_numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut first_digit: i32 = -1;
    let mut last_digit: i32 = -1;
    for i in 0..string.len() {
        if string.chars().nth(i).unwrap().is_digit(10) {
            let digit = string.chars().nth(i).unwrap().to_digit(10).unwrap() as i32;
            if first_digit == -1 {
                first_digit = digit;
            }
            last_digit = digit;
        } else {
            for j in 0..spelled_out_numbers.len() {
                if string[i..].starts_with(spelled_out_numbers[j]) {
                    if first_digit == -1 {
                        first_digit = j as i32;
                    }
                    last_digit = j as i32;
                }
            }
        }
    }

    first_digit * 10 + last_digit
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut total = 0;
    for line in lines {
        let num = _return_first_digit_and_last_digit_from_string(&line);
        total += num;
        println!("{} {}", num, line);
    }
    println!("Total: {}", total);
}
