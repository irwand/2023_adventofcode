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
    let mut time_list = Vec::new();
    let mut distance_list = Vec::new();
    let mut games: Vec<(u64, u64)> = Vec::new();
    let mut iter = lines.iter();
    iter.next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .for_each(|x| time_list.push(x));
    iter.next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .for_each(|x| distance_list.push(x));

    // #[cfg(part2)]
    {
        let time_str = time_list.join("");
        let distance_str = distance_list.join("");
        println!("{} {}", time_str, distance_str);
        games.push((
            time_list.join("").parse().unwrap(),
            distance_list.join("").parse().unwrap(),
        ))
    }
    #[cfg(part1)]
    {
        for i in time_list.iter().zip(distance_list.iter()) {
            let (time, distance) = i;
            games.push((time.parse().unwrap(), distance.parse().unwrap()));
        }
    }

    let mut total = 1;
    for (time, distance) in games.iter() {
        let mid_point = (*time as f64) / 2.0;
        let range = (((time * time) - (4 * distance)) as f64).sqrt();
        let low = (mid_point - (range / 2.0)).floor();
        let high = (mid_point + (range / 2.0)).floor();
        let adjustment = if (low as u64 * time) - (low as u64 * low as u64) == *distance {
            1
        } else {
            0
        };
        println!("{} {} {} {}", mid_point, range, low, high);

        let rounded_range = (high - low) as u64 - adjustment;
        println!("{} {} {}", time, distance, rounded_range);
        total *= rounded_range;
    }
    println!("{}", total);
}
