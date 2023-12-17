use itertools::Itertools;
use regex::Regex;
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

fn _count_consecutive_hashes(line: &str) -> Vec<u32> {
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

fn _count_char(line: &str, character: char) -> usize {
    line.chars().filter(|&c| c == character).count() as usize
}

fn _sanity_check(sequence: &str, vec_num: &Vec<u32>) -> bool {
    let min_str_len: usize = if vec_num.len() == 0 {
        0
    } else {
        vec_num.iter().copied().sum::<u32>() as usize + vec_num.len() - 1
    };
    if sequence.len() < min_str_len {
        println!("{} < {}", sequence.len(), min_str_len);
        return false;
    }

    let total_hashes = vec_num.iter().copied().sum::<u32>() as usize;
    let num_hashes = _count_char(&sequence, '#');
    if total_hashes < num_hashes {
        println!("total_hashes {} < {}", total_hashes, num_hashes);
        return false;
    }

    if sequence.starts_with('#') && vec_num.len() > 0 {
        for (index, c) in sequence.chars().enumerate() {
            if (index as u32) < vec_num[0] {
                if c != '#' && c != '?' {
                    return false;
                }
            } else if index as u32 == vec_num[0] {
                if c != '.' && c != '?' {
                    return false;
                }
            } else if index as u32 > vec_num[0] {
                break;
            }
        }
    }

    if sequence.ends_with('#') && vec_num.len() > 0 {
        let last_num = vec_num[vec_num.len() - 1];
        for (index, c) in sequence.chars().rev().enumerate() {
            if (index as u32) < last_num {
                if c != '#' && c != '?' {
                    return false;
                }
            } else if index as u32 == last_num {
                if c != '.' && c != '?' {
                    return false;
                }
            } else if index as u32 > last_num {
                break;
            }
        }
    }

    return true;
}

fn _calc_possibilities(sequence: &str, vec_num: &Vec<u32>) -> u64 {
    let mut possibilities: u64 = 0;

    println!("_calc_possibilities {} {:?}", sequence, vec_num);

    if !_sanity_check(sequence, vec_num) {
        return 0;
    }

    let total_hashes = vec_num.iter().copied().sum::<u32>() as usize;
    let num_hashes = _count_char(&sequence, '#');
    if vec_num.len() == 0 && num_hashes == 0 {
        return 1;
    }

    if vec_num.len() == 1 && num_hashes == vec_num[0] as usize {
        return 1;
    }

    // reduce amount of unknowns
    let mut reduced: String = String::new();
    if sequence.starts_with('#') || sequence.ends_with('#') {
        println!("reducing: {}", sequence);
        if sequence.starts_with('#') {
            for (index, c) in sequence.chars().enumerate() {
                if (index as u32) < vec_num[0] {
                    if c == '#' || c == '?' {
                        reduced.push('#');
                    } else {
                        return 0;
                    }
                } else if index as u32 == vec_num[0] {
                    if c == '.' || c == '?' {
                        reduced.push('.');
                    } else {
                        return 0;
                    }
                } else {
                    reduced.push(c);
                }
            }
        } else {
            reduced = sequence.to_string();
        }
        println!("reduced wip: {}", reduced);
        if sequence.ends_with('#') {
            let mut reduced_rev = String::new();
            for (index, c) in reduced.chars().rev().enumerate() {
                if (index as u32) < vec_num[vec_num.len() - 1] {
                    assert!(c == '#' || c == '?'); // already sanity checked
                    reduced_rev.push('#');
                } else if index as u32 == vec_num[vec_num.len() - 1] {
                    assert!(c == '.' || c == '?'); // already sanity checked
                    reduced_rev.push('.');
                } else {
                    reduced_rev.push(c);
                }
            }
            reduced = reduced_rev.chars().rev().collect();
        }
        println!("reduced: {}", reduced);
    }

    let sequence = if reduced.len() > 0 {
        reduced.as_str()
    } else {
        sequence
    };

    let num_hashes = _count_char(&sequence, '#');
    let re = Regex::new(r"([^\.]+)([\.]+)(.+$)").unwrap();
    if let Some(caps) = re.captures(sequence) {
        let first = caps.get(1).map_or("", |m| m.as_str());
        let _middle = caps.get(2).map_or("", |m| m.as_str());
        let last = caps.get(3).map_or("", |m| m.as_str());

        //println!("regex: {} {} {}", first, middle, last);
        for i in 0..=vec_num.len() {
            let vec_num_left = vec_num[0..i].to_vec();
            let vec_num_right = vec_num[i..].to_vec();
            if !(_sanity_check(first, &vec_num_left) && _sanity_check(last, &vec_num_right)) {
                println!(
                    "skipping {} {:?} {} {:?}",
                    first, &vec_num_left, last, &vec_num_right
                );
                continue;
            }
            let possibilities_left = _calc_possibilities(first, &vec_num_left);
            println!("left {} {:?} {}", first, vec_num_left, possibilities_left);
            if possibilities_left == 0 {
                continue;
            }
            let possibilities_right = _calc_possibilities(last, &vec_num_right);
            println!(
                "right {} {:?} {}",
                first, vec_num_right, possibilities_right
            );
            if possibilities_right == 0 {
                continue;
            }
            possibilities += possibilities_left * possibilities_right;
        }
    } else {
        let num_question_marks = _count_char(&sequence, '?');
        let hash_needed = total_hashes - num_hashes;

        if num_question_marks == 0 {
            let counts = _count_consecutive_hashes(sequence);
            if counts == *vec_num {
                return 1;
            } else {
                return 0;
            }
        }
        for i in (0..num_question_marks).combinations(hash_needed) {
            let dot_hash: Vec<char> = (0..num_question_marks)
                .map(|x| if i.contains(&x) { '#' } else { '.' })
                .collect();
            // println!("{:?} ", dot_hash);
            let mut sequence = sequence.to_string();
            for c in dot_hash.iter() {
                sequence = sequence.replacen('?', &c.to_string(), 1);
            }
            let counts = _count_consecutive_hashes(&sequence);
            if counts == *vec_num {
                possibilities += 1;
                // println!("{:04} {}", possibilities, sequence);
            }
        }
    }
    possibilities
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());

    let mut possibilities_per_line_orig: Vec<u64> = Vec::new();
    let mut possibilities_per_line_next: Vec<u64> = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let mut split = line.split_whitespace();
        let sequence = split.next().unwrap().to_string();
        let mut vec_num: Vec<u32> = split
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let possibilities = _calc_possibilities(sequence.as_str(), &vec_num);
        println!("{} {} {:?} {}", line_num, sequence, vec_num, possibilities);
        assert!(possibilities > 0);
        possibilities_per_line_orig.push(possibilities);

        // let sequence_copy: String = "?".to_string() + sequence.as_str();
        // let sequence_copy: String = sequence + "?";
        let sequence_copy = [sequence.to_owned(), sequence.to_owned()].join("?");
        vec_num = vec_num.repeat(2);
        let possibilities = _calc_possibilities(sequence_copy.as_str(), &vec_num);
        println!(
            "{} {} {:?} {}",
            line_num, sequence_copy, vec_num, possibilities
        );
        assert!(possibilities > 0);
        possibilities_per_line_next.push(possibilities);
    }
    let mut total_possibilities: u64 = 0;
    for (first, second) in possibilities_per_line_orig
        .iter()
        .zip(possibilities_per_line_next.iter())
    {
        let factor = second / first;
        // let factor = second;
        let addition = first * factor * factor * factor * factor;
        println!("{} {} {}", first, second, addition);
        total_possibilities += addition;
    }
    println!("total possibilities {}", total_possibilities);
}
