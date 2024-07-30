use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Copy, Clone)]
pub struct Hand {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Hand {
    pub fn power(&self) -> usize {self.red * self.green * self.blue}
}

fn parse_game(game_hand: &str) -> Hand {
    let re_line = Regex::new(r" (\d+) (red|green|blue)").unwrap();

    let balls: Vec<&str> = game_hand.split(",").collect();

    let mut hand = Hand {
        red: 0,
        green: 0,
        blue: 0,
    };

    for ball in balls {
        for (_, [num, color]) in re_line.captures(ball).map(|c| c.extract()) {
            match color {
                "red" => hand.red = num.parse().unwrap(),
                "green" => hand.green = num.parse().unwrap(),
                "blue" => hand.blue = num.parse().unwrap(),
                _ => (),
            }
        }
    }

    hand
}

pub fn valid_game(line: String, max_vals: Hand) -> (usize, bool) {
    let splitted = line.split(":");
    let valid = splitted.clone()
        .nth(1)
        .unwrap()
        .split(";")
        .map(parse_game)
        .map(|h| valid_hand(h, max_vals))
        .all(std::convert::identity);
    let id = splitted.clone().nth(0).unwrap()[5..].parse().unwrap();
    (id, valid)
}

pub fn get_power(line: String) -> usize {
    line.split(":")
        .nth(1)
        .unwrap()
        .split(";")
        .map(parse_game)
        .reduce(min_balls_in_game)
        .unwrap().power()
}

pub fn min_balls_in_game(this: Hand, other: Hand) -> Hand {
    Hand {
        red: max(this.red, other.red),
        green: max(this.green, other.green),
        blue: max(this.blue, other.blue),
    }
}

fn valid_hand(h: Hand, max_vals: Hand) -> bool {
    h.red <= max_vals.red && h.green <= max_vals.green && h.blue <= max_vals.blue
}

#[test]
fn test_valid_game() {
    let expected = [(1, true), (2, true), (3, false), (4, false), (5, true)];
    let max_vals = Hand {
        red: 12,
        green: 13,
        blue: 14,
    };
    if let Ok(lines) = read_lines("../testinput") {
        let result: Vec<(usize, bool)> = lines.flatten().map(|l| valid_game(l, max_vals)).collect();
        assert_eq!(result, expected)
    }
}