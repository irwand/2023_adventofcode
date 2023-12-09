use std::char;
use std::collections::HashMap;
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

fn _is_star(character: char) -> bool {
    character == '*'
}

fn _near_symbol(
    line_num: usize,
    index: usize,
    pre_line: &String,
    line: &String,
    next_line: &String,
    sym_check: fn(char) -> bool,
) -> Result<(usize, usize), i32> {
    if sym_check(pre_line.chars().nth(index).unwrap()) {
        return Ok((line_num - 1, index));
    }
    if sym_check(next_line.chars().nth(index).unwrap()) {
        return Ok((line_num + 1, index));
    }
    if sym_check(line.chars().nth(index + 1).unwrap()) {
        return Ok((line_num, index + 1));
    }
    if sym_check(pre_line.chars().nth(index + 1).unwrap()) {
        return Ok((line_num - 1, index + 1));
    }
    if sym_check(next_line.chars().nth(index + 1).unwrap()) {
        return Ok((line_num + 1, index + 1));
    }
    if sym_check(line.chars().nth(index - 1).unwrap()) {
        return Ok((line_num, index - 1));
    }
    if sym_check(pre_line.chars().nth(index - 1).unwrap()) {
        return Ok((line_num - 1, index - 1));
    }
    if sym_check(next_line.chars().nth(index - 1).unwrap()) {
        return Ok((line_num + 1, index - 1));
    }
    Err(0)
}

fn _sum_number_near_symbol(pre_line: &String, line: &String, next_line: &String) -> u32 {
    let mut total: u32 = 0;
    let mut found_digit = false;
    let mut found_symbol = false;
    let mut current_num: u32 = 0;
    for (index, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            current_num = current_num * 10 + character.to_digit(10).unwrap();
            if let Ok(_) = _near_symbol(0, index, pre_line, line, next_line, _is_symbol) {
                found_symbol |= true
            }
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

fn _populate_number_near_star(
    line_num: usize,
    pre_line: &String,
    line: &String,
    next_line: &String,
    gears: &mut HashMap<(usize, usize), Vec<u32>>,
) {
    let mut found_digit = false;
    let mut found_symbol = false;
    let mut current_num: u32 = 0;
    let mut star_coord: (usize, usize) = (0, 0);
    for (index, character) in line.chars().enumerate() {
        if character.is_digit(10) {
            current_num = current_num * 10 + character.to_digit(10).unwrap();
            if let Ok(coord) = _near_symbol(line_num, index, pre_line, line, next_line, _is_star) {
                found_symbol |= true;
                star_coord = coord;
            }
            if !found_digit {
                found_digit = true;
            }
        } else {
            if found_digit {
                if found_symbol {
                    match gears.get_mut(&star_coord) {
                        Some(nums) => {
                            nums.push(current_num);
                        }
                        None => {
                            gears.insert(star_coord, vec![current_num]);
                        }
                    }
                }
                found_digit = false;
                found_symbol = false;
                current_num = 0;
            }
        }
    }
    if found_digit {
        if found_symbol {
            match gears.get_mut(&star_coord) {
                Some(nums) => {
                    nums.push(current_num);
                }
                None => {
                    gears.insert(star_coord, vec![current_num]);
                }
            }
        }
    }
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut total: u32 = 0;
    let mut gears = HashMap::new();
    for index in 0..lines.len() {
        let line_length = lines[index].len();
        let imaginary_line = ".".repeat(line_length).to_string();
        if index == 0 {
            _populate_number_near_star(
                index,
                &imaginary_line,
                &lines[index],
                &lines[index + 1],
                &mut gears,
            );
        } else if index == lines.len() - 1 {
            _populate_number_near_star(
                index,
                &lines[index - 1],
                &lines[index],
                &imaginary_line,
                &mut gears,
            );
        } else {
            _populate_number_near_star(
                index,
                &lines[index - 1],
                &lines[index],
                &lines[index + 1],
                &mut gears,
            );
        }
        // total += number;
        // println!("{} {}", total, lines[index]);
    }
    gears.into_iter().all(|(_, nums)| {
        if nums.len() == 2 {
            total += nums[0] * nums[1];
        }
        true
    });
    println!("{}", total);
}
