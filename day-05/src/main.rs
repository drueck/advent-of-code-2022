// Advent of Code 2022: Day 5
// https://adventofcode.com/2022/day/5
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

use day_05::Move;

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

fn apply_moves_9000(stacks: &mut [Vec<char>], moves: &Vec<Move>) {
    for moov in moves {
        for _ in 0..moov.quantity {
            let krate = stacks[moov.from - 1].pop().expect("invalid move");
            stacks[moov.to - 1].push(krate)
        }
    }
}

fn apply_moves_9001(stacks: &mut [Vec<char>], moves: &Vec<Move>) {
    for moov in moves {
        let from_len = stacks[moov.from - 1].len();
        for i in (from_len - moov.quantity)..from_len {
            stacks[moov.to - 1].push(stacks[moov.from - 1][i]);
        }
        stacks[moov.from - 1].truncate(from_len - moov.quantity);
    }
}

fn top_of_stacks(stacks: &[Vec<char>]) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut parts = input.trim_end().split("\n\n");

    (
        parse_stacks(parts.next().expect("missing stacks descriptions")),
        parse_moves(parts.next().expect("missing moves descriptions")),
    )
}

fn parse_stacks(stacks_descriptions: &str) -> Vec<Vec<char>> {
    let mut lines = stacks_descriptions.split('\n').rev();
    let stack_numbers = lines
        .next()
        .expect("invalid stacks description")
        .trim()
        .as_bytes();

    // we expect at most 9 stacks
    let num_stacks = (stack_numbers.last().unwrap() - b'0') as usize;
    let mut stacks = vec![vec![]; num_stacks];

    for line in lines {
        let bytes = line.as_bytes();
        for (i, stack) in stacks.iter_mut().enumerate() {
            match bytes.get(1 + (i * 4)) {
                Some(byte) if *byte != b' ' => stack.push(*byte as char),
                _ => continue,
            }
        }
    }

    stacks
}

fn parse_moves(move_descriptions: &str) -> Vec<Move> {
    move_descriptions.split('\n').map(Move::new).collect()
}

#[test]
fn test_parse_input() {
    let input = fs::read_to_string("test-input.txt").expect("failed to read input");
    let (stacks, moves) = parse_input(&input);

    let expected_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
    let expected_moves = vec![
        Move {
            quantity: 1,
            from: 2,
            to: 1,
        },
        Move {
            quantity: 3,
            from: 1,
            to: 3,
        },
        Move {
            quantity: 2,
            from: 2,
            to: 1,
        },
        Move {
            quantity: 1,
            from: 1,
            to: 2,
        },
    ];

    assert_eq!(stacks, expected_stacks);
    assert_eq!(moves, expected_moves);
}

#[test]
fn crate_mover_9000() {
    let input = fs::read_to_string("test-input.txt").expect("failed to read input");
    let (mut stacks, moves) = parse_input(&input);
    apply_moves_9000(&mut stacks, &moves);
    assert_eq!(top_of_stacks(&stacks), "CMZ");
}

#[test]
fn crate_mover_9001() {
    let input = fs::read_to_string("test-input.txt").expect("failed to read input");
    let (mut stacks, moves) = parse_input(&input);
    apply_moves_9001(&mut stacks, &moves);
    assert_eq!(top_of_stacks(&stacks), "MCD");
}
