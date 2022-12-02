use std::{
    fs,
    io::{BufRead, BufReader},
};

enum Move {
    Rock,
    Paper,
    Scissors,
}

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;
const LOSS: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

struct Match {
    opponent: Move,
    me: Move,
}

impl Match {
    fn play(&self) -> u32 {
        match (&self.opponent, &self.me) {
            (Move::Rock, Move::Rock) => ROCK + DRAW,
            (Move::Rock, Move::Paper) => PAPER + WIN,
            (Move::Rock, Move::Scissors) => SCISSORS + LOSS,
            (Move::Paper, Move::Rock) => ROCK + LOSS,
            (Move::Paper, Move::Paper) => PAPER + DRAW,
            (Move::Paper, Move::Scissors) => SCISSORS + WIN,
            (Move::Scissors, Move::Rock) => ROCK + WIN,
            (Move::Scissors, Move::Paper) => PAPER + LOSS,
            (Move::Scissors, Move::Scissors) => SCISSORS + DRAW,
        }
    }
}

fn main() {
    let strategy_file = fs::File::open("data/day2.txt").expect("Could not find data file");
    let mut scores = Vec::new();

    for line in BufReader::new(strategy_file).lines() {
        match line {
            Ok(l) => {
                let moves: Vec<&str> = l.trim().split(" ").collect();
                let their_move = match moves[0] {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    "C" => Move::Scissors,
                    _ => panic!("Not a valid move for opponent"),
                };
                let my_move = match moves[1] {
                    "X" => Move::Rock,
                    "Y" => Move::Paper,
                    "Z" => Move::Scissors,
                    _ => panic!("Not a valid move for me"),
                };
                let match_ = Match {
                    opponent: their_move,
                    me: my_move,
                };
                scores.push(match_.play());
            }
            Err(_) => println!("Uh oh! Couldn't read the moves"),
        }
    }

    println!("My Score: {}", scores.into_iter().sum::<u32>())
}
