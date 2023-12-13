use itertools::{Itertools, MultiProduct};
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

fn _count_consecutive_hashes(line: &String) -> Vec<u32> {
    let mut count = 0;
    let mut counts = Vec::new();
    for c in line.chars() {
        if c == '#' {
            count += 1;
        } else {
            if count > 0 {
                counts.push(count);
            }
            count = 0
        }
    }
    if count > 0 {
        counts.push(count);
    }
    counts
}

fn _count_question_mark(line: &String) -> u32 {
    line.chars().filter(|&c| c == '?').count() as u32
}

/// Rust version of Python's itertools.product().
/// It returns the cartesian product of the input iterables, and it is
/// semantically equivalent to `repeat` nested for loops.
///
/// # Arguments
///
/// * `it` - An iterator over a cloneable data structure
/// * `repeat` - Number of repetitions of the given iterator
pub fn product_repeat<I>(it: I, repeat: usize) -> MultiProduct<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    std::iter::repeat(it).take(repeat).multi_cartesian_product()
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());

    let mut possibilities = 0;
    for line in lines {
        let mut split = line.split_whitespace();
        let sequence = split.next().unwrap().to_string();
        let vec_num: Vec<u32> = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        println!("{} {:?}", sequence, vec_num);

        let num_question_marks = _count_question_mark(&sequence);

        for i in product_repeat(['.', '#'].iter(), num_question_marks as usize) {
            let mut sequence = sequence.clone();
            for c in i.iter() {
                sequence = sequence.replacen('?', &c.to_string(), 1);
            }
            let counts = _count_consecutive_hashes(&sequence);
            if counts == vec_num {
                possibilities += 1;
                // println!("{} {}", possibilities, sequence);
            }
        }
    }
    println!("total possibilities {}", possibilities);
}
