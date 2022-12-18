// Advent of Code 2022: Day 13
// https://adventofcode.com/2022/day/13
// Usage: `cargo run <input-file>`

use day_13::Element;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let pairs = parse_input(&input);

    println!("The answer to part 1 is {}", part_1(&pairs));
    println!("The answer to part 2 is {}", part_2(&pairs));
}

fn parse_input(input: &str) -> Vec<(Element, Element)> {
    input
        .trim()
        .split("\n\n")
        .map(|pair_of_lines| {
            let mut lines = pair_of_lines.split('\n');
            (
                Element::new(lines.next().unwrap()),
                Element::new(lines.next().unwrap()),
            )
        })
        .collect()
}

fn part_1(element_pairs: &[(Element, Element)]) -> usize {
    element_pairs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (first, second))| {
            acc + if first < second { i + 1 } else { 0 }
        })
}

fn part_2(pairs: &[(Element, Element)]) -> usize {
    let mut elements: Vec<&Element> = Vec::with_capacity(pairs.len() * 2 + 2);
    let markers = [Element::new("[[2]]"), Element::new("[[6]]")];

    for (first, second) in pairs {
        elements.push(first);
        elements.push(second);
    }
    elements.push(&markers[0]);
    elements.push(&markers[1]);

    elements.sort_unstable();

    markers
        .iter()
        .map(|marker| elements.iter().position(|e| *e == marker).unwrap() + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let pairs = parse_input(&input);
        assert_eq!(part_1(&pairs), 13);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let pairs = parse_input(&input);
        assert_eq!(part_2(&pairs), 140);
    }
}
