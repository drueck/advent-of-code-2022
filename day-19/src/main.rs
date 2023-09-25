// Advent of Code 2022: Day 19
// https://adventofcode.com/2022/day/19
// Usage: `cargo run <input-file>`

use day_19::Blueprint;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input file");
    let _blueprints = parse_input(&input);

    todo!()
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .trim()
        .split('\n')
        .map(|line| line.parse().unwrap())
        .collect()
}
