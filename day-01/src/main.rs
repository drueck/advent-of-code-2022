// Advent of Code 2022: Day 1
// https://adventofcode.com/2022/day/1
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input");

    let mut ordered_calories: Vec<usize> = input
        .trim()
        .split("\n\n")
        .map(|items| {
            items
                .split('\n')
                .map(|calories| calories.parse::<usize>().expect("invalid integer"))
                .sum()
        })
        .collect();

    ordered_calories.sort_by(|a, b| b.cmp(a));

    let top = ordered_calories[0];
    let top_three: usize = ordered_calories.into_iter().take(3).sum();

    println!("The elf with the most calories was carrying: {}", top);
    println!("The top three combined were carrying: {}", top_three);
}
