// Advent of Code 2022: Day 9
// https://adventofcode.com/2022/day/9
// Usage: `cargo run <input-file>`

use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let head_moves = parse_head_moves(&input);

    println!(
        "Unique tail locations: {}",
        unique_tail_locations(&head_moves, 2)
    );

    println!(
        "Unique long tail locations: {}",
        unique_tail_locations(&head_moves, 10)
    );
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    offset: (isize, isize),
    count: usize,
}

impl Move {
    fn new(s: &str) -> Self {
        let mut parts = s.split(' ');
        let offset = match parts.next().unwrap() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("invalid direction char"),
        };
        let count = parts.next().unwrap().parse().unwrap();
        Self { offset, count }
    }
}

fn parse_head_moves(input: &str) -> Vec<Move> {
    input.trim().split('\n').map(Move::new).collect()
}

fn unique_tail_locations(moves: &[Move], num_knots: usize) -> usize {
    let mut knots = vec![(0, 0); num_knots];
    let mut locations = HashSet::new();

    for m in moves {
        for _ in 0..m.count {
            knots[0] = (knots[0].0 + m.offset.0, knots[0].1 + m.offset.1);
            for i in 1..num_knots {
                let tail_move = tail_move(knots[i - 1], knots[i]);
                knots[i] = (knots[i].0 + tail_move.0, knots[i].1 + tail_move.1);
            }
            locations.insert(knots[num_knots - 1]);
        }
    }

    locations.len()
}

fn tail_move(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (x, y) if x.abs() > 1 || y.abs() > 1 => (sign(x), sign(y)),
        _ => (0, 0),
    }
}

// convert into -1, 0, or 1
fn sign(val: isize) -> isize {
    if val == 0 {
        return val;
    }
    val / val.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_tail_locations() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read test input");
        let moves = parse_head_moves(&input);
        assert_eq!(unique_tail_locations(&moves, 2), 13);
    }

    #[test]
    fn test_unique_long_tail_locations() {
        let input = fs::read_to_string("test-input-part-2.txt").expect("failed to read test input");
        let moves = parse_head_moves(&input);
        assert_eq!(unique_tail_locations(&moves, 10), 36);
    }
}
