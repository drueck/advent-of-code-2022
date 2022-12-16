// Advent of Code 2022: Day 12
// https://adventofcode.com/2022/day/12
// Usage: `cargo run <input-file>`

use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let grid: Vec<&[u8]> = input.trim().split('\n').map(|s| s.as_bytes()).collect();

    println!(
        "The quickest route from start to end is {}",
        fewest_steps(&grid)
    );
}

fn fewest_steps(grid: &[&[u8]]) -> usize {
    let mut queue = VecDeque::new();
    let mut steps = HashMap::new();
    let mut end = (0, 0);

    // - find the starting space
    for (rowi, row) in grid.iter().enumerate() {
        for (coli, val) in row.iter().enumerate() {
            match *val {
                b'S' => {
                    steps.insert((rowi, coli), 0);
                    queue.push_back((rowi, coli));
                }
                b'E' => {
                    end = (rowi, coli);
                }
                _ => {
                    continue;
                }
            }
        }
    }

    let mut possible_moves: [Option<(usize, usize)>; 4] = [None; 4];

    while let Some((row, col)) = queue.pop_front() {
        if (row, col) == end {
            continue;
        }

        let next_steps = steps[&(row, col)] + 1;

        update_possible_moves(grid, &mut possible_moves, row, col);

        for (adj_row, adj_col) in possible_moves.iter().flatten() {
            match steps.get_mut(&(*adj_row, *adj_col)) {
                // if we've explored this space before, update the cheapest
                // path if the current one is cheaper
                Some(prev_steps) => {
                    if next_steps < *prev_steps {
                        *prev_steps = next_steps;
                    }
                }
                // if we've never explored this space, record the current steps
                // and add it to the queue to explore later
                None => {
                    steps.insert((*adj_row, *adj_col), next_steps);
                    queue.push_back((*adj_row, *adj_col));
                }
            }
        }
    }

    steps[&end]
}

fn update_possible_moves(
    grid: &[&[u8]],
    possible_moves: &mut [Option<(usize, usize)>; 4],
    row: usize,
    col: usize,
) {
    let max_row_index = grid.len() - 1;
    let max_col_index = grid[0].len() - 1;
    let here = match grid[row][col] {
        b'S' => b'a',
        n => n,
    };

    // top
    possible_moves[0] = match row > 0 && legal_move(here as i8, grid[row - 1][col] as i8) {
        true => Some((row - 1, col)),
        false => None,
    };

    // right
    possible_moves[1] =
        match col < max_col_index && legal_move(here as i8, grid[row][col + 1] as i8) {
            true => Some((row, col + 1)),
            false => None,
        };

    // bottom
    possible_moves[2] =
        match row < max_row_index && legal_move(here as i8, grid[row + 1][col] as i8) {
            true => Some((row + 1, col)),
            false => None,
        };

    // left
    possible_moves[3] = match col > 0 && legal_move(here as i8, grid[row][col - 1] as i8) {
        true => Some((row, col - 1)),
        false => None,
    };
}

fn legal_move(from: i8, to: i8) -> bool {
    let adj_from = if from as u8 == b'S' { b'a' as i8 } else { from };
    let adj_to = if to as u8 == b'E' { b'z' as i8 } else { to };
    adj_to - adj_from < 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fewest_steps() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let grid: Vec<&[u8]> = input.trim().split('\n').map(|s| s.as_bytes()).collect();
        assert_eq!(fewest_steps(&grid), 31);
    }

    #[test]
    fn test_update_possible_moves() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let grid: Vec<&[u8]> = input.trim().split('\n').map(|s| s.as_bytes()).collect();

        let mut possible_moves = [None; 4];
        update_possible_moves(&grid, &mut possible_moves, 0, 0);

        assert_eq!(possible_moves, [None, Some((0, 1)), Some((1, 0)), None]);
    }
}
