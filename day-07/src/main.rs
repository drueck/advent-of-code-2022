// Advent of Code 2022: Day 7
// https://adventofcode.com/2022/day/7
// Usage: `cargo run <input-file>`

use day_07::{build_directories, part_1, part_2};
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let directories = build_directories(&input);

    println!(
        "The sum of directories with at most 100000 bytes is: {}",
        part_1(&directories)
    );

    println!(
        "The size of the directory that should be deleted is: {}",
        part_2(&directories)
    );
}
