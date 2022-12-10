// Advent of Code 2022: Day 10
// https://adventofcode.com/2022/day/10
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

const CRT_LINE_LENGTH: usize = 40;
const CRT_LINES: usize = 6;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let instructions = parse_instructions(&input);

    println!(
        "Sum of signal strengths: {}",
        sum_of_signal_strengths(&instructions)
    );

    render_crt(&instructions);
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

fn execute<F>(instructions: &[Instruction], mut side_effect: F)
where
    F: FnMut(isize, isize),
{
    let mut x = 1;
    let mut cycles = 1;

    for instruction in instructions {
        match instruction {
            Instruction::Noop => {
                side_effect(cycles, x);
                cycles += 1;
            }
            Instruction::Addx(n) => {
                side_effect(cycles, x);
                cycles += 1;
                side_effect(cycles, x);
                x += n;
                cycles += 1;
            }
        }
    }
}

fn sum_of_signal_strengths(instructions: &[Instruction]) -> isize {
    let mut signal_strengths = 0;

    execute(instructions, |cycles, x| {
        if cycles % 40 == 20 {
            signal_strengths += cycles * x;
        }
    });

    signal_strengths
}

fn render_crt(instructions: &[Instruction]) {
    let mut crt = [b' '; CRT_LINE_LENGTH * CRT_LINES];

    execute(instructions, |cycles, x| {
        let draw_index = (cycles % (CRT_LINE_LENGTH as isize)) - 1;
        if x - 1 <= draw_index && draw_index <= x + 1 {
            crt[cycles as usize - 1] = b'#';
        }
    });

    for line in crt.chunks_exact(40) {
        println!("{}", std::str::from_utf8(line).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_signal_strengths() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let instructions = parse_instructions(&input);
        assert_eq!(sum_of_signal_strengths(&instructions), 13140);
    }
}
