// Advent of Code 2022: Day 10
// https://adventofcode.com/2022/day/10
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let instructions = parse_instructions(&input);

    println!("The answer to part 1 is: {}", part_1(&instructions));
}

enum Instruction {
    Addx(isize),
    Noop,
}

impl Instruction {
    fn new(s: &str) -> Self {
        match s {
            "noop" => Self::Noop,
            addx => Self::Addx(addx.split(' ').nth(1).unwrap().parse().unwrap()),
        }
    }
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.trim().split('\n').map(Instruction::new).collect()
}

fn part_1(instructions: &[Instruction]) -> isize {
    let mut x = 1;
    let mut signal_strengths = 0;
    let mut cycles = 1;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                maybe_record_signal_strength(&mut signal_strengths, cycles, x);
                cycles += 1;
            }
            Instruction::Addx(n) => {
                maybe_record_signal_strength(&mut signal_strengths, cycles, x);
                cycles += 1;
                maybe_record_signal_strength(&mut signal_strengths, cycles, x);
                x += n;
                cycles += 1;
            }
        }
    }
    signal_strengths
}

fn maybe_record_signal_strength(signal_strengths: &mut isize, cycles: isize, x: isize) {
    if cycles % 40 == 20 {
        *signal_strengths += cycles * x;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let instructions = parse_instructions(&input);
        assert_eq!(part_1(&instructions), 13140);
    }
}
