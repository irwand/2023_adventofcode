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

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let expansion: u64 = 1000000;

    let mut empty_y: Vec<usize> = Vec::new();
    for i in 0..lines.len() {
        if lines[i].chars().all(|c| c == '.') {
            empty_y.push(i);
        }
    }

    let mut empty_x: Vec<usize> = Vec::new();
    for i in 0..lines[0].len() {
        if lines
            .iter()
            .map(|x| x.chars().nth(i).unwrap())
            .all(|c| c == '.')
        {
            empty_x.push(i);
        }
    }

    // detect position of all #
    let mut positions: Vec<(usize, usize)> = Vec::new();
    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            if lines[y].chars().nth(x).unwrap() == '#' {
                positions.push((x, y));
            }
        }
    }

    // calculate sum of all distances
    let mut sum: u64 = 0;
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let (x1, y1) = positions[i];
            let (x2, y2) = positions[j];

            let min_x = x1.min(x2);
            let max_x = x1.max(x2);
            let empty_x_num: u64 = empty_x
                .iter()
                .filter(|x| **x > min_x && **x < max_x)
                .count() as u64;

            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            let empty_y_num: u64 = empty_y
                .iter()
                .filter(|y| **y > min_y && **y < max_y)
                .count() as u64;

            sum += ((max_x - min_x) as u64 + (empty_x_num * (expansion - 1)))
                + (max_y - min_y) as u64
                + (empty_y_num * (expansion - 1));
        }
    }

    println!("{}", sum);
}
