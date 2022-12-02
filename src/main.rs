use std::{
    fs,
    io::{BufRead, BufReader},
};

enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Loss,
    Draw,
    Win,
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

struct MatchUnencrypted {
    opponent: Move,
    me: Outcome,
}

impl MatchUnencrypted {
    fn play(&self) -> u32 {
        match (&self.opponent, &self.me) {
            (Move::Rock, Outcome::Loss) => SCISSORS + LOSS,
            (Move::Rock, Outcome::Draw) => ROCK + DRAW,
            (Move::Rock, Outcome::Win) => PAPER + WIN,
            (Move::Paper, Outcome::Loss) => ROCK + LOSS,
            (Move::Paper, Outcome::Draw) => PAPER + DRAW,
            (Move::Paper, Outcome::Win) => SCISSORS + WIN,
            (Move::Scissors, Outcome::Loss) => PAPER + LOSS,
            (Move::Scissors, Outcome::Draw) => SCISSORS + DRAW,
            (Move::Scissors, Outcome::Win) => ROCK + WIN,
        }
    }
}

fn play_tournament(path: &str) -> u32 {
    let file = fs::File::open(path).expect("Could not find data file");
    let mut scores = Vec::new();

    for line in BufReader::new(file).lines() {
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

    scores.into_iter().sum::<u32>()
}

fn play_tournament_unencrypted(path: &str) -> u32 {
    let file = fs::File::open(path).expect("Could not find data file");
    let mut scores = Vec::new();

    for line in BufReader::new(file).lines() {
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
                    "X" => Outcome::Loss,
                    "Y" => Outcome::Draw,
                    "Z" => Outcome::Win,
                    _ => panic!("Not a valid outcome for me"),
                };
                let match_ = MatchUnencrypted {
                    opponent: their_move,
                    me: my_move,
                };
                scores.push(match_.play());
            }
            Err(_) => println!("Uh oh! Couldn't read the moves"),
        }
    }

    scores.into_iter().sum::<u32>()
}

fn main() {
    let file_path = "data/day2.txt";
    let score = play_tournament(file_path);
    let score_unencrypted = play_tournament_unencrypted(file_path);

    println!("My Score: {}", score);
    println!("My Unencrypted Score: {}", score_unencrypted);
}
