// Advent of Code 2022: Day 6
// https://adventofcode.com/2022/day/6
// Usage: `cargo run <input-file>`

use day_06::index_after_n_unique_characters;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let datastream = fs::read_to_string(&input_filename).expect("failed to read input file");

    match index_after_n_unique_characters(datastream.as_bytes(), 4) {
        Some(result) => println!("Packet starts at: {result}"),
        None => println!("No start-of-packet marker was found!"),
    }

    match index_after_n_unique_characters(datastream.as_bytes(), 14) {
        Some(result) => println!("Message starts at: {result}"),
        None => println!("No start-of-message marker was found!"),
    }
}
