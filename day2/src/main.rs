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

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn _parse_game_line(line: &str) -> (u32, Round) {
    let mut split = line.split(":");
    let gamestr = split.next().unwrap().to_string();
    let rounds = split.next().unwrap().to_string();

    let mut split = gamestr.split(" ");
    let game = split.next().unwrap().to_string();
    assert!(game == "Game");
    let gamenum = split
        .next()
        .unwrap()
        .to_string()
        .trim()
        .parse::<u32>()
        .unwrap();

    let mut max_round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };
    let mut split = rounds.split(";");
    split.by_ref().for_each(|round| {
        let mut round_total = Round {
            red: 0,
            green: 0,
            blue: 0,
        };
        let mut split = round.split(",");
        split.by_ref().for_each(|color_str| {
            let mut split = color_str.trim().split(" ");
            let num = split
                .next()
                .unwrap()
                .to_string()
                .trim()
                .parse::<u32>()
                .unwrap();
            let color = split.next().unwrap().trim().to_string();
            match color.as_str() {
                "red" => {
                    round_total.red += num;
                }
                "green" => {
                    round_total.green += num;
                }
                "blue" => {
                    round_total.blue += num;
                }
                _ => {}
            }
        });
        println!(
            "round {} {} {}",
            round_total.red, round_total.green, round_total.blue
        );
        if round_total.red > max_round.red {
            max_round.red = round_total.red;
        }
        if round_total.green > max_round.green {
            max_round.green = round_total.green;
        }
        if round_total.blue > max_round.blue {
            max_round.blue = round_total.blue;
        }
    });
    (gamenum, max_round)
}

fn main() {
    // pass first argument to function
    let lines = _open_file_and_return_lines(std::env::args().nth(1).unwrap());
    let mut total: u32 = 0;
    for line in lines {
        let (num, max_round) = _parse_game_line(&line);
        total += max_round.red * max_round.green * max_round.blue;
        println!(
            "{} {} {} {}",
            num, max_round.red, max_round.green, max_round.blue
        );
    }
    println!("{}", total);
}
