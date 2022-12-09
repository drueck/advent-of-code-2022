// Advent of Code 2022: Day 8
// https://adventofcode.com/2022/day/8
// Usage: `cargo run <input-file>`

use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let forest = parse_input(&input);

    println!(
        "The number of trees visible from the outside are: {}",
        visible_tree_locations(&forest).len()
    );

    println!(
        "The best scenic score in the forest is: {}",
        best_scenic_score(&forest)
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
    let height = forest.len();
    let width = forest[0].len();

    // TODO: Can I make this just one pattern?
    macro_rules! find_visible {
        ($forest:ident, $visible:ident, x in $outer:expr, y in $inner:expr) => {{
            for x in $outer {
                let mut tallest_so_far = b'0' - 1;
                for y in $inner {
                    let height = forest[y][x];
                    if height > tallest_so_far {
                        tallest_so_far = height;
                        visible.insert((x, y));
                    }
                }
            }
        }};
        ($forest:ident, $visible:ident, y in $outer:expr, x in $inner:expr) => {{
            for y in $outer {
                let mut tallest_so_far = b'0' - 1;
                for x in $inner {
                    let height = forest[y][x];
                    if height > tallest_so_far {
                        tallest_so_far = height;
                        visible.insert((x, y));
                    }
                }
            }
        }};
    }

    find_visible![forest, visible, x in 0..width, y in 0..height - 1]; // top
    find_visible![forest, visible, y in 0..height, x in (1..width).rev()]; // right
    find_visible![forest, visible, x in 0..width, y in (1..height).rev()]; // bottom
    find_visible![forest, visible, y in 0..height, x in 0..width]; // left

    visible
}

fn scenic_score(forest: &[&[u8]], x: usize, y: usize) -> usize {
    let viewing_height = forest[y][x];
    let height = forest.len();
    let width = forest[0].len();

    // TODO: Can I make this just one pattern?
    macro_rules! count_visible {
        ($x:ident, y in $y_range:expr) => {{
            let mut visible = 0;
            for yi in $y_range {
                visible += 1;
                if forest[yi][$x] >= viewing_height {
                    break;
                }
            }
            visible
        }};
        ($y:ident, x in $x_range:expr) => {{
            let mut visible = 0;
            for xi in $x_range {
                visible += 1;
                if forest[y][xi] >= viewing_height {
                    break;
                }
            }
            visible
        }};
    }

    let visible_up = count_visible![x, y in (0..y).rev()];
    let visible_right = count_visible![y, x in min(x + 1, width)..width];
    let visible_down = count_visible![x, y in min(y + 1, height)..height];
    let visible_left = count_visible![y, x in (0..x).rev()];

    visible_up * visible_right * visible_down * visible_left
}

fn best_scenic_score(forest: &[&[u8]]) -> usize {
    let mut best = 0;
    let width = forest[0].len();

    for y in 0..forest.len() {
        for x in 0..width {
            let score = scenic_score(forest, x, y);
            if score > best {
                best = score;
            }
        }
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_visible_trees() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read test input");
        let forest = parse_input(&input);
        let locations = visible_tree_locations(&forest);

        assert_eq!(locations.len(), 21);
    }

    #[test]
    fn test_scenic_score() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read test input");
        let forest = parse_input(&input);

        assert_eq!(scenic_score(&forest, 2, 1), 4);
        assert_eq!(scenic_score(&forest, 2, 3), 8);
    }

    #[test]
    fn test_best_scenic_score() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read test input");
        let forest = parse_input(&input);

        assert_eq!(best_scenic_score(&forest), 8);
    }
}
