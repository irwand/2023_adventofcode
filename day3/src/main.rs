use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn _open_file_and_return_lines(filepath: String) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(".".to_string() + line.unwrap().as_str() + ".");
    }

    lines // Return the lines vector
}

fn _is_symbol(character: char) -> bool {
    character != '.' && !(character.is_digit(10))
}

fn _near_symbol(index: usize, pre_line: &String, line: &String, next_line: &String) -> bool {
    if _is_symbol(pre_line.chars().nth(index).unwrap()) {
        return true;
    }
    if _is_symbol(next_line.chars().nth(index).unwrap()) {
        return true;
    }
    if _is_symbol(line.chars().nth(index + 1).unwrap()) {
        return true;
    }
    if _is_symbol(pre_line.chars().nth(index + 1).unwrap()) {
        return true;
    }
    if _is_symbol(next_line.chars().nth(index + 1).unwrap()) {
        return true;
    }
    if _is_symbol(line.chars().nth(index - 1).unwrap()) {
        return true;
    }
    if _is_symbol(pre_line.chars().nth(index - 1).unwrap()) {
        return true;
    }
    if _is_symbol(next_line.chars().nth(index - 1).unwrap()) {
        return true;
    }
    false
}

fn _sum_number_near_symbol(pre_line: &String, line: &String, next_line: &String) -> u32 {
    let mut total: u32 = 0;
    let mut found_digit = false;
    let mut found_symbol = false;
    let mut current_num: u32 = 0;
    for (index, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            current_num = current_num * 10 + character.to_digit(10).unwrap();
            found_symbol |= _near_symbol(index, pre_line, line, next_line);
            if !found_digit {
                found_digit = true;
            }
        } else {
            if found_digit {
                if found_symbol {
                    total = total + current_num;
                    println!("found: {} {}", current_num, total);
                }
                found_digit = false;
                found_symbol = false;
                current_num = 0;
            }
        }
    }
    if found_digit {
        if found_symbol {
            total = total + current_num;
            println!("found: {} {}", current_num, total);
        }
    }
    total
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut total: u32 = 0;
    for index in 0..lines.len() {
        let line_length = lines[index].len();
        let imaginary_line = ".".repeat(line_length).to_string();
        let number: u32;
        if index == 0 {
            number = _sum_number_near_symbol(&imaginary_line, &lines[index], &lines[index + 1]);
        } else if index == lines.len() - 1 {
            number = _sum_number_near_symbol(&lines[index - 1], &lines[index], &imaginary_line);
        } else {
            number = _sum_number_near_symbol(&lines[index - 1], &lines[index], &lines[index + 1]);
        }
        total += number;
        println!("{} {}", total, lines[index]);
    }
    println!("{}", total);
}
