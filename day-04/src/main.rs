// Advent of Code 2022: Day 4
// https://adventofcode.com/2022/day/4
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let (fully_contained, overlapping) = count_overlaps(&input);

    println!("Number of assignments fully contained within each other: {fully_contained}");
    println!("Number of overlapping assignments: {overlapping}");
}

fn parse_ranges(line: &str) -> (usize, usize, usize, usize) {
    let mut iter = line
        .split(['-', ','])
        .map(|s| s.parse::<usize>().expect("invalid int"));

    (
        iter.next().expect("malformed range"),
        iter.next().expect("malformed range"),
        iter.next().expect("malformed range"),
        iter.next().expect("malformed range"),
    )
}

fn count_overlaps(input: &str) -> (usize, usize) {
    let assignments = input.trim().split('\n').map(parse_ranges);
    let mut num_fully_contained = 0;
    let mut num_overlapping = 0;

    for (min_a, max_a, min_b, max_b) in assignments {
        if fully_contained(min_a, max_a, min_b, max_b) {
            num_fully_contained += 1;
        }
        if overlapping(min_a, max_a, min_b, max_b) {
            num_overlapping += 1;
        }
    }

    (num_fully_contained, num_overlapping)
}

fn fully_contained(min_a: usize, max_a: usize, min_b: usize, max_b: usize) -> bool {
    min_a >= min_b && max_a <= max_b || min_b >= min_a && max_b <= max_a
}

fn overlapping(min_a: usize, max_a: usize, min_b: usize, max_b: usize) -> bool {
    !(max_a < min_b || max_b < min_a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fully_contained() {
        assert!(!fully_contained(2, 4, 6, 8));
        assert!(fully_contained(2, 8, 3, 7));
    }

    #[test]
    fn test_overlapping() {
        assert!(!overlapping(2, 4, 6, 8));
        assert!(overlapping(5, 7, 7, 9));
    }

    #[test]
    fn test_count_overlaps() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read test input");
        assert_eq!(count_overlaps(&input), (2, 4));
    }
}
