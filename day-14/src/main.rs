// Advent of Code 2022: Day 14
// https://adventofcode.com/2022/day/14
// Usage: `cargo run <input-file>`

use day_14::Cave;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let mut cave = Cave::new(&input);

    let mut cave_with_floor = cave.clone();
    cave_with_floor.build_floor();

    while cave.drop_sand(500, 0).is_some() {}
    println!(
        "The number of grains that came to rest in the cave with no floor were {}",
        cave.grains_at_rest
    );

    while cave_with_floor.drop_sand(500, 0).is_some() {}
    println!(
        "The number of grains that came to rest in the cave with no floor were {}",
        cave_with_floor.grains_at_rest
    );
}
