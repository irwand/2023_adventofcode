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

fn _rotate_terrain(terrain: &Vec<String>) -> Vec<String> {
    let mut rotated_terrain: Vec<String> = Vec::new();
    for i in 0..terrain[0].len() {
        let mut line = String::new();
        for j in 0..terrain.len() {
            line.push(terrain[terrain.len() - 1 - j].chars().nth(i).unwrap());
        }
        rotated_terrain.push(line);
    }
    rotated_terrain
}
fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut total = 0;
    let rotated_terrain: Vec<String> = _rotate_terrain(&lines);
    // rotated_terrain.iter().for_each(|x| println!("{}", x));
    let mut tilted_terrain: Vec<String> = Vec::new();
    for line in rotated_terrain {
        let mut segments: Vec<String> = Vec::new();
        line.split("#").for_each(|x| {
            let mut sorted: Vec<char> = x.chars().collect::<Vec<char>>();
            sorted.sort();
            segments.push(sorted.into_iter().collect());
        });
        let tilted_line = segments.join("#");
        tilted_line.chars().enumerate().for_each(|(i, c)| {
            if c == 'O' {
                total += i + 1;
            }
        });
        tilted_terrain.push(tilted_line);
    }
    // tilted_terrain.iter().for_each(|x| println!("{}", x));
    println!("Total: {}", total);
}
