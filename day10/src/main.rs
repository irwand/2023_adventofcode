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

fn get_next_move(from: char, pipe: char) -> (char, (i32, i32)) {
    match pipe {
        '-' => match from {
            'E' => ('E', (1, 0)),
            'W' => ('W', (-1, 0)),
            _ => panic!("Invalid direction"),
        },
        '|' => match from {
            'N' => ('N', (0, 1)),
            'S' => ('S', (0, -1)),
            _ => panic!("Invalid direction"),
        },
        '7' => match from {
            'E' => ('N', (0, 1)),
            'S' => ('W', (-1, 0)),
            _ => panic!("Invalid direction"),
        },
        'J' => match from {
            'E' => ('S', (0, -1)),
            'N' => ('W', (-1, 0)),
            _ => panic!("Invalid direction"),
        },
        'F' => match from {
            'W' => ('N', (0, 1)),
            'S' => ('E', (1, 0)),
            _ => panic!("Invalid direction"),
        },
        'L' => match from {
            'W' => ('S', (0, -1)),
            'N' => ('E', (1, 0)),
            _ => panic!("Invalid direction"),
        },
        _ => panic!("Invalid direction"),
    }
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());

    let mut position: (i32, i32) = (0, 0);
    for (index, line) in lines.iter().enumerate() {
        if let Some(col) = line.chars().position(|x| x == 'S') {
            println!("Found S at line {} and column {}", index, col);
            position = (col as i32, index as i32);
        }
    }

    let mut paths: Vec<(usize, usize)> = Vec::new();
    let mut from = 'S';
    let mut next_move = (0, -1);
    let mut total = 1;
    position.0 += next_move.0;
    position.1 += next_move.1;
    paths.push((position.0 as usize, position.1 as usize));
    let mut next = lines[position.1 as usize]
        .chars()
        .nth(position.0 as usize)
        .unwrap();
    while next != 'S' {
        (from, next_move) = get_next_move(from, next);
        position.0 += next_move.0;
        position.1 += next_move.1;
        paths.push((position.0 as usize, position.1 as usize));
        next = lines[position.1 as usize]
            .chars()
            .nth(position.0 as usize)
            .unwrap();
        total += 1;
    }
    println!(
        "Total moves: {}, furthest: {}",
        paths.len(),
        paths.len() / 2
    );
}
