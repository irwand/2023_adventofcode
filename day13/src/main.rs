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

fn _compare_lines(line1: &str, line2: &str) -> u32 {
    let mut diff = 0;
    for i in 0..line1.len() {
        if line1.chars().nth(i).unwrap() != line2.chars().nth(i).unwrap() {
            diff += 1;
        }
    }
    println!("{} {} {}", line1, line2, diff);
    diff
}

fn _find_symmetry_horizontal(terrain: &Vec<&str>, num_diff: u32) -> u64 {
    println!("horizontal: {:?}", terrain);
    for i in 0..terrain.len() - 1 {
        let line1 = terrain[i];
        let line2 = terrain[i + 1];

        let mut total_num_diff = num_diff;
        let diff = _compare_lines(line1, line2);
        if diff <= total_num_diff {
            total_num_diff -= diff;
            let mut sym1 = i;
            let mut sym2 = i + 1;
            let mut reached_end = true;
            while sym1 > 0 && sym2 < terrain.len() - 1 {
                println!("start reflection {} {}", sym1, sym2);
                sym1 -= 1;
                sym2 += 1;
                let diff = _compare_lines(terrain[sym1], terrain[sym2]);
                if diff > total_num_diff {
                    println!("break {}", diff);
                    reached_end = false;
                    break;
                } else {
                    total_num_diff -= diff;
                }
            }
            if reached_end {
                if total_num_diff == 0 {
                    println!("success {}", diff);
                    return (i + 1) as u64;
                }
            }
        }
    }
    0
}

fn _find_symmetry_vertical(terrain: &Vec<&str>, num_diff: u32) -> u64 {
    let mut rotated_terrain: Vec<String> = Vec::new();
    for i in 0..terrain[0].len() {
        let mut line = String::new();
        for j in 0..terrain.len() {
            line.push(terrain[j].chars().nth(i).unwrap());
        }
        rotated_terrain.push(line);
    }
    _find_symmetry_horizontal(
        &(rotated_terrain
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()),
        num_diff,
    )
}

fn _find_symmetry(terrain: &Vec<&str>, num_diff: u32) -> u64 {
    let sym_horizontal = _find_symmetry_horizontal(terrain, num_diff);
    if sym_horizontal > 0 {
        println!("Horizontal symmetry: {}", sym_horizontal * 100);
        return (sym_horizontal * 100) as u64;
    }

    let sym_vertical = _find_symmetry_vertical(terrain, num_diff);
    if sym_vertical > 0 {
        println!("Vertical symmetry: {}", sym_vertical);
        return sym_vertical as u64;
    }
    assert!(false, "shouldn't reach here");
    0
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut total = 0;
    let mut terrain: Vec<&str> = Vec::new();
    for line in lines.iter().map(|s| s.as_str()) {
        if line.len() == 0 {
            let num = _find_symmetry(&terrain, 1);
            total += num;
            println!("{}", total);
            terrain.clear();
        } else {
            terrain.push(line);
        }
    }
    let num = _find_symmetry(&terrain, 1);
    total += num;
    println!("Total: {}", total);
}
