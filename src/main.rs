use std::{fs, io};

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Loss(u32),
    Draw(u32),
    Win(u32),
}

const ROCK: u32 = 0;
const PAPER: u32 = 3;
const SCISSORS: u32 = 6;
const LOSS: u32 = 0;
const DRAW: u32 = 3;
const WIN: u32 = 6;

struct Match {
    player_1_move: Move,
    player_2_move: Move,
}

impl Match {
    fn play(&self) -> Outcome {
        match (self.player_1_move, self.player_2_move) {
            (Move::Rock, Move::Rock) => Outcome::Draw(ROCK + ROCK + DRAW),
            (Move::Rock, Move::Paper) => Outcome::Loss(ROCK + PAPER + LOSS),
            (Move::Rock, Move::Scissors) => Outcome::Win(ROCK + SCISSORS + WIN),
            (Move::Paper, Move::Rock) => Outcome::Win(PAPER + ROCK + WIN),
            (Move::Paper, Move::Paper) => Outcome::Draw(PAPER + PAPER + DRAW),
            (Move::Paper, Move::Scissors) => Outcome::Loss(PAPER + SCISSORS + LOSS),
            (Move::Scissors, Move::Rock) => Outcome::Loss(SCISSORS + ROCK + LOSS),
            (Move::Scissors, Move::Paper) => Outcome::Win(SCISSORS + PAPER + WIN),
            (Move::Scissors, Move::Scissors) => Outcome::Draw(SCISSORS + SCISSORS + DRAW),
        }
    }
}

fn main() {
    let strategy_file = fs::File::open("data/day2.txt").expect("Could not find data file");
    let mut buffer = io::BufReader::new(strategy_file);
}
