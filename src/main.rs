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
    player_1_move: Move,
    player_2_move: Move,
}

impl Match {
    fn play(&self) -> u32 {
        match (&self.player_1_move, &self.player_2_move) {
            (Move::Rock, Move::Rock) => ROCK + ROCK + DRAW,
            (Move::Rock, Move::Paper) => ROCK + PAPER + LOSS,
            (Move::Rock, Move::Scissors) => ROCK + SCISSORS + WIN,
            (Move::Paper, Move::Rock) => PAPER + ROCK + WIN,
            (Move::Paper, Move::Paper) => PAPER + PAPER + DRAW,
            (Move::Paper, Move::Scissors) => PAPER + SCISSORS + LOSS,
            (Move::Scissors, Move::Rock) => SCISSORS + ROCK + LOSS,
            (Move::Scissors, Move::Paper) => SCISSORS + PAPER + WIN,
            (Move::Scissors, Move::Scissors) => SCISSORS + SCISSORS + DRAW,
        }
    }
}

fn main() {
    let strategy_file = fs::File::open("data/day2-2.txt").expect("Could not find data file");
    let mut scores = Vec::new();

    for line in BufReader::new(strategy_file).lines() {
        match line {
            Ok(l) => {
                let moves: Vec<&str> = l.trim().split(" ").collect();
                let my_move = match moves[0] {
                    "A" => Move::Rock,
                    "B" => Move::Paper,
                    "C" => Move::Scissors,
                    _ => panic!("Not a valid move for player 1"),
                };
                let their_move = match moves[1] {
                    "X" => Move::Rock,
                    "Y" => Move::Paper,
                    "Z" => Move::Scissors,
                    _ => panic!("Not a valid move for player 2"),
                };
                let match_ = Match {
                    player_1_move: my_move,
                    player_2_move: their_move,
                };
                scores.push(match_.play());
            }
            Err(_) => println!("Uh oh! Couldn't read the moves"),
        }
    }

    println!("My Score: {}", scores.into_iter().sum::<u32>())
}
