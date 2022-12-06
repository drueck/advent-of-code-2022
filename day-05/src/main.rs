// Advent of Code 2022: Day 5
// https://adventofcode.com/2022/day/5
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

use day_05::{apply_moves_9000, apply_moves_9001, parse_input, top_of_stacks};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");

    let (mut stacks_part_1, moves) = parse_input(&input);
    let mut stacks_part_2 = stacks_part_1.clone();

    apply_moves_9000(&mut stacks_part_1, &moves);
    apply_moves_9001(&mut stacks_part_2, &moves);

    println!(
        "Appying the moves with CrateMover 9000 we get: {}",
        top_of_stacks(&stacks_part_1)
    );

    println!(
        "Appying the moves with CrateMover 9001 we get: {}",
        top_of_stacks(&stacks_part_2)
    );
}
