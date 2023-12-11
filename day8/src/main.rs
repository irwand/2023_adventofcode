use std::collections::HashMap;
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

    let direction = lines[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();

    let mut map = HashMap::new();
    for line in lines[2..].iter() {
        let mut split = line.split("=");
        let loc = split.next().unwrap().trim().to_string();
        let dests = split.next().unwrap().trim().to_string();
        let dest_tuple = dests[1..dests.len() - 1]
            .split(",")
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>();
        map.insert(loc, dest_tuple);
    }

    let mut whereamis: Vec<&String> = map.keys().filter(|k| k.ends_with("A")).collect();
    let mut totals: Vec<u64> = Vec::new();
    for w in whereamis {
        let mut whereami = w;
        let mut total: u64 = 0;
        println!("start: {}", whereami);
        for i in direction.iter().cycle() {
            total += 1;
            let dest: &String = &map[whereami][*i];
            if dest.ends_with('Z') {
                break;
            }
            whereami = dest;
        }
        println!("total: {}", total);
        totals.push(total);
    }

    println!("{}", totals.iter().fold(1, |acc, x| acc * x));
}
