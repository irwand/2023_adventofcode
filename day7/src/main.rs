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

struct Hand {
    cards: String,
    bid: u64,
}

impl Hand {
    fn new(cards: String, bid: u64) -> Hand {
        Hand { cards, bid }
    }

    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        self.bid.cmp(&other.bid)
    }
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut total: u64 = 0;
    let mut hands = Vec::new();

    for line in lines {
        let mut cards: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        let bid = cards.pop().unwrap().parse::<u64>().unwrap();
        let hand = Hand::new(cards.pop().unwrap(), bid);
        println!("{} {}", hand.cards, hand.bid);
        hands.push(hand);
    }

    println!("{}", total);
}
