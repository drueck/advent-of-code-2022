// Advent of Code 2022: Day 6
// https://adventofcode.com/2022/day/6
// Usage: `cargo run <input-file>`

use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let datastream = fs::read_to_string(&input_filename).expect("failed to read input file");

    match index_after_n_unique_characters(datastream.as_bytes(), 4) {
        Some(result) => println!("Packet starts at: {result}"),
        None => println!("No start-of-packet marker was found!"),
    }

    match index_after_n_unique_characters(datastream.as_bytes(), 14) {
        Some(result) => println!("Message starts at: {result}"),
        None => println!("No start-of-message marker was found!"),
    }
}

fn index_after_n_unique_characters(datastream: &[u8], n: usize) -> Option<usize> {
    let mut candidate = vec![0; n];
    for (i, group) in datastream.windows(n).enumerate() {
        candidate.copy_from_slice(group);
        candidate.sort();
        if candidate.windows(2).all(|chars| chars[0] != chars[1]) {
            return Some(i + n);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_after_start_of_packet() {
        let test_data = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];

        for (input, output) in test_data {
            assert_eq!(
                index_after_n_unique_characters(input.as_bytes(), 4),
                Some(output)
            );
        }
    }

    #[test]
    fn test_index_after_start_of_message() {
        let test_data = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];

        for (input, output) in test_data {
            assert_eq!(
                index_after_n_unique_characters(input.as_bytes(), 14),
                Some(output)
            );
        }
    }
}
