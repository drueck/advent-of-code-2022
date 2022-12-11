// Advent of Code 2022: Day 11
// https://adventofcode.com/2022/day/11
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
}
