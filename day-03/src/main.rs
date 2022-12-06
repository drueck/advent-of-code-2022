// Advent of Code 2022: Day 3
// https://adventofcode.com/2022/day/3
// Usage: `cargo run <input-file>`

use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let lines: Vec<_> = input.trim().split('\n').collect();

    println!("The answer for part 1 is {}", part_1(&lines));
    println!("The answer for part 2 is {}", part_2(&lines));
}

fn part_1(lines: &[&str]) -> usize {
    let mut total = 0;
    for line in lines {
        let (first, second) = line.split_at(line.len() / 2);
        let first_set: HashSet<char> = first.chars().collect();
        let second_set: HashSet<char> = second.chars().collect();
        let common_type = first_set
            .intersection(&second_set)
            .next()
            .expect("we expect one common type between compartments");
        total += priority(*common_type);
    }
    total
}

fn part_2(lines: &[&str]) -> usize {
    let mut total = 0;

    for group in lines.chunks_exact(3) {
        let mut first: HashSet<char> = group[0].chars().collect();
        let second: HashSet<char> = group[1].chars().collect();
        let third: HashSet<char> = group[2].chars().collect();

        first.retain(|t| second.contains(t));
        first.retain(|t| third.contains(t));

        let common_type = first
            .iter()
            .next()
            .expect("we expect one common type per group");

        total += priority(*common_type);
    }

    total
}

fn priority(type_char: char) -> usize {
    (match type_char as u8 {
        lower if (b'a'..=b'z').contains(&lower) => lower - b'a' + 1,
        upper if (b'A'..=b'Z').contains(&upper) => upper - b'A' + 27,
        _ => panic!("unexpected type char"),
    }) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        let types_priorities = vec![
            ('p', 16),
            ('L', 38),
            ('P', 42),
            ('v', 22),
            ('t', 20),
            ('s', 19),
        ];

        for (type_char, expected_priority) in types_priorities {
            assert_eq!(priority(type_char), expected_priority);
        }
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input");
        let lines: Vec<_> = input.trim().split('\n').collect();
        assert_eq!(part_1(&lines), 157);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input");
        let lines: Vec<_> = input.trim().split('\n').collect();
        assert_eq!(part_2(&lines), 70);
    }
}
