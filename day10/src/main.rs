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
    let mut lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());

    let mut position: (i32, i32) = (0, 0);
    for (index, line) in lines.iter().enumerate() {
        if let Some(col) = line.chars().position(|x| x == 'S') {
            println!("Found S at line {} and column {}", index, col);
            position = (col as i32, index as i32);
        }
    }
    let s_coord = position;

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

    let mut paths_copy = paths.clone();
    paths_copy.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    // 'S' is 'J'
    assert!(
        lines[s_coord.1 as usize]
            .chars()
            .nth(s_coord.0 as usize)
            .unwrap()
            == 'S'
    );
    lines[s_coord.1 as usize] = lines[s_coord.1 as usize].replacen("S", "J", 1);
    println!("{}", lines[s_coord.1 as usize]);
    let mut tiles: u32 = 0;
    for y in 0..lines.len() {
        let paths_this_line: Vec<usize> = paths_copy
            .iter()
            .filter(|i| i.1 == y)
            .map(|i| i.0)
            .collect();

        let mut in_area = false;
        let mut look_for = '0';
        for x in 0..lines[y].len() {
            if paths_this_line.contains(&x) {
                let char_here = lines[y].chars().nth(x).unwrap();
                if char_here == '|' {
                    in_area = !in_area;
                } else if char_here == '-' {
                    if look_for == '0' {
                        panic!(
                            "Invalid path, char_here: {}, look_for: {}, x: {}, y: {}",
                            char_here, look_for, x, y
                        );
                    }
                } else if char_here == look_for {
                    in_area = !in_area;
                    look_for = '0';
                } else if char_here == 'L' && look_for == '0' {
                    look_for = '7';
                } else if char_here == 'F' && look_for == '0' {
                    look_for = 'J';
                } else if char_here == 'J' && look_for == '7' {
                    look_for = '0';
                } else if char_here == '7' && look_for == 'J' {
                    look_for = '0';
                } else {
                    panic!(
                        "Invalid path, char_here: {}, look_for: {}, x: {}, y: {}",
                        char_here, look_for, x, y
                    );
                }
            } else {
                if in_area {
                    tiles += 1;
                }
            }
        }
    }
    println!("Total tiles: {}", tiles);
}
