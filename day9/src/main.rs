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

fn _find_next_value(sequence: &Vec<i64>) -> i64 {
    let mut diff = Vec::new();
    for i in 0..sequence.len() - 1 {
        diff.push(sequence[i + 1] - sequence[i]);
    }
    if diff.iter().all(|x| *x == 0) {
        0
    } else {
        let next_value = _find_next_value(&diff);
        let retval = next_value + diff[diff.len() - 1];
        retval
    }
}

fn _find_prev_value(sequence: &Vec<i64>) -> i64 {
    let mut diff = Vec::new();
    for i in 0..sequence.len() - 1 {
        diff.push(sequence[i + 1] - sequence[i]);
    }
    if diff.iter().all(|x| *x == 0) {
        0
    } else {
        let prev_value = _find_prev_value(&diff);
        let retval = diff[0] - prev_value;
        retval
    }
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut last_numbers = Vec::new();

    for line in lines {
        let sequence: Vec<i64> = line
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let prev_value = _find_prev_value(&sequence);
        let last_number = sequence[0] - prev_value;
        println!("{}", last_number);
        last_numbers.push(last_number);
    }

    println!("Sum: {}", last_numbers.iter().sum::<i64>());
}
