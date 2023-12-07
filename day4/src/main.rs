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

fn _get_score_from_game(line: &String) -> u32 {
    let mut split = line.split(":");
    let _ = split.next().unwrap();
    let rest = split.next().unwrap();

    let mut split = rest.split("|");
    let mut winning_numbers: Vec<u32> = Vec::new();
    let winning_numbers_str = split.next().unwrap();
    for winning_number_str in winning_numbers_str.trim().split(" ") {
        if let Ok(number) = winning_number_str.parse::<u32>() {
            winning_numbers.push(number);
        }
    }

    let mut given_numbers: Vec<u32> = Vec::new();
    let current_numbers_str = split.next().unwrap();
    for current_number_str in current_numbers_str.trim().split(" ") {
        if let Ok(number) = current_number_str.parse::<u32>() {
            given_numbers.push(number);
        }
    }

    let mut score: u32 = 0;
    for winning_number in winning_numbers.iter() {
        for given_number in given_numbers.iter() {
            if winning_number == given_number {
                score += 1;
            }
        }
    }

    score
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut total: u32 = 0;
    let mut card: usize = 0;
    let mut total_cards: Vec<u32> = vec![0; 219];
    for line in lines.iter() {
        total_cards[card] += 1;
        let number: usize = _get_score_from_game(line) as usize;
        println!("{} {}", number, line);
        for i in 1..=number {
            total_cards[card + i] += total_cards[card];
        }
        card += 1
    }
    total_cards.into_iter().for_each(|x| total += x);
    println!("{}", total);
}
