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

    // expand all dot rows and columns
    let mut expanded_lines: Vec<String> = Vec::new();
    for i in lines.into_iter() {
        if i.chars().all(|c| c == '.') {
            expanded_lines.push(i.clone());
        }
        expanded_lines.push(i);
    }

    let mut i: usize = 0;
    while i < expanded_lines[0].len() {
        if expanded_lines
            .iter()
            .map(|x| x.chars().nth(i).unwrap())
            .all(|c| c == '.')
        {
            for j in 0..expanded_lines.len() {
                expanded_lines[j].insert(i, '.');
            }
            i += 1;
        }
        i += 1;
    }

    // detect position of all #
    let mut positions: Vec<(usize, usize)> = Vec::new();
    for y in 0..expanded_lines.len() {
        for x in 0..expanded_lines[y].len() {
            if expanded_lines[y].chars().nth(x).unwrap() == '#' {
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
            sum += ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as u64;
        }
    }

    println!("{}", sum);
}
