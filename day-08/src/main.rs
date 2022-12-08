// Advent of Code 2022: Day 8
// https://adventofcode.com/2022/day/8
// Usage: `cargo run <input-file>`

use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let forest = parse_input(&input);
    let locations = visible_tree_locations(&forest);

    println!(
        "The number of trees visible from the outside are: {}",
        locations.len()
    );
}

fn parse_input(input: &str) -> Vec<&[u8]> {
    input
        .trim()
        .split('\n')
        .map(|line| line.as_bytes())
        .collect()
}

fn visible_tree_locations(forest: &[&[u8]]) -> HashSet<(usize, usize)> {
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // from the top
    for x in 0..forest[0].len() {
        let mut tallest_so_far = b'0' - 1;
        // the bottom ones are always visible, and we'll catch them when we start from the bottom
        for y in 0..forest.len() - 1 {
            let height = forest[y][x];
            if height > tallest_so_far {
                tallest_so_far = height;
                visible.insert((x, y));
            }
        }
    }

    // from the right
    for y in 0..forest.len() {
        let mut tallest_so_far = b'0' - 1;
        // the left ones are always visible, we'll catch them when we start from the left
        for x in (1..forest[0].len()).rev() {
            let height = forest[y][x];
            if height > tallest_so_far {
                tallest_so_far = height;
                visible.insert((x, y));
            }
        }
    }

    // from the bottom
    for x in 0..forest[0].len() {
        let mut tallest_so_far = b'0' - 1;
        // the top ones are always visible, and we already captured those
        for y in (1..forest.len()).rev() {
            let height = forest[y][x];
            if height > tallest_so_far {
                tallest_so_far = height;
                visible.insert((x, y));
            }
        }
    }

    // from the left
    for y in 0..forest.len() {
        let mut tallest_so_far = b'0' - 1;
        // the right ones are always visible, and we already captured those
        for x in 0..forest[0].len() - 1 {
            let height = forest[y][x];
            if height > tallest_so_far {
                tallest_so_far = height;
                visible.insert((x, y));
            }
        }
    }

    visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_visible_trees() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read test input");
        let forest = parse_input(&input);
        let locations = visible_tree_locations(&forest);

        let expected_locations = HashSet::from([
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (1, 1), // top left 5
            (2, 1), // top middle 5
            (1, 2), // left middle 5
            (3, 2), // right middle 3
            (2, 3), // bottom row middle 5
        ]);

        assert_eq!(locations, expected_locations);
        assert_eq!(locations.len(), 21);
    }
}
