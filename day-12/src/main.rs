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

    let start = locations(&grid, 'S')[0];
    let end = locations(&grid, 'E')[0];
    let mut aes = locations(&grid, 'a');
    aes.push(start);

    match fewest_steps(&grid, start, end) {
        Some(n) => {
            println!("The shortest route from start to end is {n} steps",);
        }
        None => {
            println!("Couldn't find a route from the start to the end!");
        }
    }

    let shortest_a_to_end = aes
        .into_iter()
        .flat_map(|a| fewest_steps(&grid, a, end))
        .min()
        .unwrap();

    println!(
        "The shortest route from any a to the end is {} steps",
        shortest_a_to_end
    );
}

// returns the locations of the given char in the grid
fn locations(grid: &[&[u8]], c: char) -> Vec<(usize, usize)> {
    let mut results = vec![];
    for (rowi, row) in grid.iter().enumerate() {
        for (coli, val) in row.iter().enumerate() {
            if *val == c as u8 {
                results.push((rowi, coli));
            }
        }
    }
    results
}

fn fewest_steps(grid: &[&[u8]], start: (usize, usize), end: (usize, usize)) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut steps = HashMap::new();
    let mut possible_moves: [Option<(usize, usize)>; 4] = [None; 4];

    steps.insert(start, 0);
    queue.push_back(start);

    while let Some((row, col)) = queue.pop_front() {
        if (row, col) == end {
            continue;
        }

        let current_steps: usize = steps[&(row, col)] + 1;

        update_possible_moves(grid, &mut possible_moves, row, col);

        for (adj_row, adj_col) in possible_moves.into_iter().flatten() {
            match steps.get_mut(&(adj_row, adj_col)) {
                Some(prev_steps) => {
                    if current_steps < *prev_steps {
                        *prev_steps = current_steps;
                    }
                }
                None => {
                    steps.insert((adj_row, adj_col), current_steps);
                    queue.push_back((adj_row, adj_col));
                }
            }
        }
    }

    steps.remove(&end)
}

fn update_possible_moves(
    grid: &[&[u8]],
    possible_moves: &mut [Option<(usize, usize)>; 4],
    row: usize,
    col: usize,
) {
    let max_row_index = grid.len() - 1;
    let max_col_index = grid[0].len() - 1;
    let here = grid[row][col];

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

fn translate(height: i8) -> i8 {
    match height as u8 {
        b'S' => b'a' as i8,
        b'E' => b'z' as i8,
        n => n as i8,
    }
}

fn legal_move(from: i8, to: i8) -> bool {
    translate(to) - translate(from) < 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let grid: Vec<&[u8]> = input.trim().split('\n').map(|s| s.as_bytes()).collect();
        let start = locations(&grid, 'S')[0];
        let end = locations(&grid, 'E')[0];
        assert_eq!(fewest_steps(&grid, start, end), Some(31));
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let grid: Vec<&[u8]> = input.trim().split('\n').map(|s| s.as_bytes()).collect();
        let aes = locations(&grid, 'a');
        let end = locations(&grid, 'E')[0];

        let shortest_a_to_end: usize = aes
            .into_iter()
            .flat_map(|a| fewest_steps(&grid, a, end))
            .min()
            .unwrap();

        assert_eq!(shortest_a_to_end, 29);
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
