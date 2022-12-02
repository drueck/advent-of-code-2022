// Advent of Code 2022: Day 2
// https://adventofcode.com/2022/day/2
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let score_part_1 = play(&input, parse_strategy_guide_part_1);
    let score_part_2 = play(&input, parse_strategy_guide_part_2);

    println!("For part 1 your score would be: {score_part_1}");
    println!("For part 2 your score would be: {score_part_2}");
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum RPSResult {
    Loss = 0,
    Tie = 3,
    Win = 6,
}

impl RPS {
    fn vs(&self, other: &RPS) -> RPSResult {
        match (self, other) {
            (RPS::Rock, RPS::Paper) => RPSResult::Loss,
            (RPS::Rock, RPS::Scissors) => RPSResult::Win,
            (RPS::Paper, RPS::Rock) => RPSResult::Win,
            (RPS::Paper, RPS::Scissors) => RPSResult::Loss,
            (RPS::Scissors, RPS::Rock) => RPSResult::Loss,
            (RPS::Scissors, RPS::Paper) => RPSResult::Win,
            _ => RPSResult::Tie,
        }
    }
}

fn parse_strategy_guide_part_1(input: &str) -> Vec<(RPS, RPS)> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let round: Vec<_> = line
                .split(' ')
                .map(|char| match char {
                    "A" | "X" => RPS::Rock,
                    "B" | "Y" => RPS::Paper,
                    "C" | "Z" => RPS::Scissors,
                    _ => {
                        panic!("invalid RPS character!");
                    }
                })
                .collect();
            (round[0], round[1])
        })
        .collect()
}

fn parse_strategy_guide_part_2(input: &str) -> Vec<(RPS, RPS)> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            let round_input: Vec<_> = line.split(' ').collect();
            let opponent = match round_input[0] {
                "A" => RPS::Rock,
                "B" => RPS::Paper,
                "C" => RPS::Scissors,
                _ => panic!("invalid RPS character!"),
            };
            let player = match (opponent, round_input[1]) {
                (RPS::Rock, "X") => RPS::Scissors,
                (RPS::Paper, "X") => RPS::Rock,
                (RPS::Scissors, "X") => RPS::Paper,
                (RPS::Rock, "Z") => RPS::Paper,
                (RPS::Paper, "Z") => RPS::Scissors,
                (RPS::Scissors, "Z") => RPS::Rock,
                (any, "Y") => any,
                _ => panic!("unexpected pattern"),
            };
            (opponent, player)
        })
        .collect()
}

fn play(strategy_guide: &str, parse: for<'r> fn(&'r str) -> Vec<(RPS, RPS)>) -> usize {
    let mut score = 0;
    for (opponent, player) in parse(&strategy_guide) {
        score += player.vs(&opponent) as usize + player as usize;
    }
    score
}

#[test]
fn part_1() {
    let input = fs::read_to_string("test-input.txt").expect("failed to read input");
    assert_eq!(play(&input, parse_strategy_guide_part_1), 15);
}

#[test]
fn part_2() {
    let input = fs::read_to_string("test-input.txt").expect("failed to read input");
    assert_eq!(play(&input, parse_strategy_guide_part_2), 12);
}
