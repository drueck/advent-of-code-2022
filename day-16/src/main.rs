// Advent of Code 2022: Day 16
// https://adventofcode.com/2022/day/16
// Usage: `cargo run <input-file>`

use day_16::volcano::{Strategy, Volcano};
use std::env;
use std::fs;
use Strategy::{Alone, WithElephant};

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let volcano = Volcano::new(&input);

    println!(
        "The max pressure released with you working alone is: {}",
        volcano.find_best_pressure(Alone)
    );

    println!(
        "The max pressure released with you and the elephant working together is: {}",
        volcano.find_best_pressure(WithElephant)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let volcano = Volcano::new(&input);
        assert_eq!(volcano.find_best_pressure(Alone), 1651);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let volcano = Volcano::new(&input);
        assert_eq!(volcano.find_best_pressure(WithElephant), 1707);
    }
}
