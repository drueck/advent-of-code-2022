// Advent of Code 2022: Day 15
// https://adventofcode.com/2022/day/15
// Usage: `cargo run <input-file>`

use regex::Regex;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input file");
    let pairs = parse_input(&input);
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

fn parse_input(input: &str) -> Vec<(Point, Point)> {
    let d = "(-?[0-9]+)"; // intentionally not using \d because I disabled unicode in regex
    let format = format!(r"Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}");
    let re = Regex::new(&format[..]).unwrap();

    input
        .trim()
        .split('\n')
        .map(|line| {
            let numbers = re.captures(line).expect("didn't match the regex");
            let sensor = Point {
                x: numbers[1].parse().unwrap(),
                y: numbers[2].parse().unwrap(),
            };
            let beacon = Point {
                x: numbers[3].parse().unwrap(),
                y: numbers[4].parse().unwrap(),
            };
            (sensor, beacon)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let pairs = parse_input(&input);

        assert_eq!(pairs.len(), 14);
        assert_eq!(pairs[0], (Point { x: 2, y: 18 }, Point { x: -2, y: 15 }));
        assert_eq!(pairs[13], (Point { x: 20, y: 1 }, Point { x: 15, y: 3 }));
    }
}
