// Advent of Code 2022: Day 9
// https://adventofcode.com/2022/day/9
// Usage: `cargo run <input-file>`

use std::collections::HashSet;
use std::env;
use std::fs;

const NUM_TAILS: usize = 9;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let head_moves = parse_head_moves(&input);

    println!(
        "Unique tail locations: {}",
        unique_tail_locations(&head_moves)
    );

    println!(
        "Unique long tail locations: {}",
        unique_long_tail_locations(&head_moves)
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

fn unique_tail_locations(moves: &[Move]) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut locations = HashSet::new();

    for m in moves {
        for _ in 0..m.count {
            head.0 += m.offset.0;
            head.1 += m.offset.1;
            let tail_move = tail_move_from_diff(head.0 - tail.0, head.1 - tail.1);
            tail.0 += tail_move.0;
            tail.1 += tail_move.1;
            locations.insert(tail);
        }
    }

    locations.len()
}

fn unique_long_tail_locations(moves: &[Move]) -> usize {
    let mut tails = [(0, 0); NUM_TAILS];
    let mut locations = HashSet::new();
    let mut head = (0, 0);

    for m in moves {
        for _ in 0..m.count {
            head.0 += m.offset.0;
            head.1 += m.offset.1;
            let mut prev = head;

            #[allow(clippy::needless_range_loop)]
            for i in 0..NUM_TAILS {
                let tail_move = tail_move_from_diff(prev.0 - tails[i].0, prev.1 - tails[i].1);
                tails[i] = (tails[i].0 + tail_move.0, tails[i].1 + tail_move.1);
                prev = tails[i];
            }
            locations.insert(tails[NUM_TAILS - 1]);
        }
    }

    locations.len()
}

fn tail_move_from_diff(x: isize, y: isize) -> (isize, isize) {
    assert!(x.abs() < 3);
    assert!(y.abs() < 3);

    match (x, y) {
        (2, 2) => (1, 1),
        (-2, -2) => (-1, -1),
        (2, n) => (1, n),
        (-2, n) => (-1, n),
        (n, 2) => (n, 1),
        (n, -2) => (n, -1),
        _ => (0, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_tail_locations() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read test input");
        let moves = parse_head_moves(&input);
        assert_eq!(unique_tail_locations(&moves), 13);
    }

    #[test]
    fn test_unique_long_tail_locations() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read test input");
        let moves = parse_head_moves(&input);
        assert_eq!(unique_long_tail_locations(&moves), 1);
    }

    #[test]
    fn test_longer_unique_tail_locations() {
        let input = fs::read_to_string("test-input-part-2.txt").expect("failed to read test input");
        let moves = parse_head_moves(&input);
        assert_eq!(unique_long_tail_locations(&moves), 36);
    }
}
