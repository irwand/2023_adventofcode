use std::cmp::Ordering;
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

#[derive(Eq, PartialOrd, PartialEq)]
struct Hand {
    cards: String,
    bid: u64,
    poker_type: u8,
}

const HIGH_CARD: u8 = 0;
const ONE_PAIR: u8 = 1;
const TWO_PAIR: u8 = 2;
const THREE_OF_A_KIND: u8 = 3;
const FULL_HOUSE: u8 = 4;
const FOUR_OF_A_KIND: u8 = 5;
const FIVE_OF_A_KIND: u8 = 6;

fn _poker_type(in_cards: &String) -> u8 {
    let mut cards = HashMap::new();
    for card in in_cards.chars() {
        let count = cards.entry(card).or_insert(0);
        *count += 1;
    }

    if cards.len() == 5 {
        if cards.contains_key(&'J') {
            ONE_PAIR
        } else {
            HIGH_CARD
        }
    } else if cards.len() == 4 {
        if cards.contains_key(&'J') {
            THREE_OF_A_KIND
        } else {
            ONE_PAIR
        }
    } else if cards.len() == 3 {
        if cards.values().any(|&x| x == 3) {
            if cards.contains_key(&'J') {
                FOUR_OF_A_KIND
            } else {
                THREE_OF_A_KIND
            }
        } else {
            if cards.get(&'J') == Some(&2) {
                FOUR_OF_A_KIND
            } else if cards.get(&'J') == Some(&1) {
                FULL_HOUSE
            } else {
                TWO_PAIR
            }
        }
    } else if cards.len() == 2 {
        if cards.values().any(|&x| x == 4) {
            if cards.contains_key(&'J') {
                FIVE_OF_A_KIND
            } else {
                FOUR_OF_A_KIND
            }
        } else {
            if cards.contains_key(&'J') {
                FIVE_OF_A_KIND
            } else {
                FULL_HOUSE
            }
        }
    } else {
        FIVE_OF_A_KIND
    }
}

impl Hand {
    const CARD_ORDER: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    fn new(cards: String, bid: u64) -> Hand {
        let poker_type = _poker_type(&cards);
        Hand {
            cards,
            bid,
            poker_type,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        if self.poker_type != other.poker_type {
            return self.poker_type.cmp(&other.poker_type);
        } else {
            for (self_card, other_card) in self.cards.chars().zip(other.cards.chars()) {
                let self_card_index = Hand::CARD_ORDER
                    .iter()
                    .position(|&x| x == self_card)
                    .unwrap();
                let other_card_index = Hand::CARD_ORDER
                    .iter()
                    .position(|&x| x == other_card)
                    .unwrap();
                if self_card_index != other_card_index {
                    return self_card_index.cmp(&other_card_index);
                }
            }
            Ordering::Equal
        }
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

    hands.sort_by(|a, b| a.cmp(b));

    for (index, hand) in hands.iter().enumerate() {
        println!("{} {}", hand.cards, hand.bid);
        total += hand.bid * (index + 1) as u64;
    }

    println!("{}", total);
}
