// Advent of Code 2022: Day 17
// https://adventofcode.com/2022/day/17
// Usage: `cargo run <input-file>`

use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input file");
    let mut chamber = Chamber::new();

    // convert to -1 or 1 using ascii math ('<', '=', '>') are (60, 61, and 62)
    let gusts: Vec<i8> = input
        .trim()
        .as_bytes()
        .iter()
        .map(|b| *b as i8 - b'=' as i8)
        .collect();

    let rocks = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // horizontal line
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // plus
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // backward L
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // vertical line
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],         // square
    ];

    println!(
        "The answer to part 1 is: {}",
        chamber.height_after_rocks_stopped(2022, &gusts[..], &rocks)
    );

    println!(
        "The answer to part 2 is: {}",
        chamber.height_after_rocks_stopped(1000000000000, &gusts[..], &rocks)
    );
}

struct Chamber {
    pub heights: [usize; 7],
    pub current_rock: [Option<(u8, usize)>; 5],
    pub structure: HashSet<(u8, usize)>,
    pub diffs: Vec<usize>,
}

impl Chamber {
    pub fn new() -> Self {
        Self {
            heights: [0; 7],
            current_rock: [None; 5],
            structure: HashSet::new(),
            diffs: Vec::with_capacity(1024),
        }
    }

    pub fn reset(&mut self) {
        self.heights = [0; 7];
        self.current_rock = [None; 5];
        self.structure = HashSet::new();
        self.diffs = Vec::with_capacity(1024);
    }

    pub fn height_after_rocks_stopped(
        &mut self,
        num_rocks: usize,
        gusts: &[i8],
        rocks: &[Vec<(u8, usize)>],
    ) -> usize {
        self.reset();

        let mut gusts_iter = gusts.iter().cycle();
        for (rock_num, rock) in rocks.iter().cycle().enumerate() {
            if rock_num == num_rocks {
                break;
            }

            // if we have a lot of rocks, start looking for a cycle
            if rock_num % 10_000 == 0 {
                if let Some((cycle_start, cycle)) = self.try_detect_first_cycle() {
                    let height_per_cycle: usize = cycle.iter().sum();
                    let before_cycles = cycle_start;
                    let after_cycles = (num_rocks - before_cycles) % cycle.len();
                    let num_cycles = (num_rocks - before_cycles) / cycle.len();

                    let height_outside_of_cycles = self.height_after_rocks_stopped(
                        before_cycles + after_cycles,
                        &gusts,
                        &rocks,
                    );

                    return height_outside_of_cycles + (height_per_cycle * num_cycles);
                }
            }

            self.drop_rock(rock);
            loop {
                self.try_apply_gust(*gusts_iter.next().unwrap());
                if !self.try_apply_gravity() {
                    self.add_current_rock_to_structure();
                    break;
                }
            }
        }

        self.max_height()
    }

    // inserts the given rock at its starting position in self.current_rock
    // starting position is 2 over from the left
    // and 4 up from the highest rock at rest (or the floor if it's the first rock)
    fn drop_rock(&mut self, rock: &[(u8, usize)]) {
        assert!(rock.len() < 6);
        let x_base = 2;
        let y_base = self.max_height() + 4;

        self.current_rock = [None; 5];
        for (i, (x, y)) in rock.iter().enumerate() {
            self.current_rock[i] = Some((x_base + x, y_base + y));
        }
    }

    // try to apply the given gust (x offset)
    // return true if it could move, false otherwise
    fn try_apply_gust(&mut self, gust: i8) -> bool {
        if self.current_rock.iter().flatten().all(|(x, y)| {
            let new_x = *x as i8 + gust;
            new_x > -1 && new_x < 7 && !self.structure.contains(&(new_x as u8, *y))
        }) {
            for i in 0..5 {
                if let Some((x, y)) = self.current_rock[i] {
                    let new_x = (x as i8 + gust) as u8;
                    self.current_rock[i] = Some((new_x, y));
                }
            }
            return true;
        }
        false
    }

    // try to drop the ~tetrimino~ rock by one y
    // return true if it could move, false otherwise
    fn try_apply_gravity(&mut self) -> bool {
        if self
            .current_rock
            .iter()
            .flatten()
            .all(|(x, y)| *y > 1 && !self.structure.contains(&(*x, *y - 1)))
        {
            for i in 0..5 {
                if let Some((x, y)) = self.current_rock[i] {
                    self.current_rock[i] = Some((x, y - 1));
                }
            }
            return true;
        }
        false
    }

    fn add_current_rock_to_structure(&mut self) {
        let previous_max_height = self.max_height();
        for (x, y) in self.current_rock.iter().flatten() {
            self.heights[*x as usize] = self.heights[*x as usize].max(*y);
            self.structure.insert((*x, *y));
        }
        self.diffs.push(self.max_height() - previous_max_height)
    }

    pub fn max_height(&self) -> usize {
        *self.heights.iter().max().unwrap()
    }

    // returns Some(start, cycle) or None
    // where start is the index of the first occurence of the cycle and cycle is the sequence
    fn try_detect_first_cycle(&self) -> Option<(usize, Vec<usize>)> {
        // say the end of the diffs is like 12312312312
        // the cycle that try_detect_cycle will return is 312, which does repeat
        // but what we actually want is 123, which is the *first* thing that repeats
        // so we want to rotate the digits until we find our "123" and also get the first occurence
        let cycle = self.try_detect_cycle()?;
        let cycle_len = cycle.len();

        let first_occurence = |seq: &[usize]| {
            self.diffs
                .windows(cycle_len)
                .position(|w| w == seq)
                .unwrap()
        };

        let mut candidate = Vec::from(&cycle[..]);
        let mut best_start = first_occurence(&candidate);

        for _ in 0..cycle.len() {
            candidate.rotate_left(1);
            best_start = first_occurence(&candidate).min(best_start);
        }

        let best_candidate = Vec::from(&self.diffs[best_start..(best_start + cycle.len())]);
        assert_eq!(cycle.len(), best_candidate.len());

        Some((best_start, best_candidate))
    }

    // returns Some(cycle) or None where the cycle is the sequence of height diffs
    // starts at the end, assuming we're in the cycle and looks for repeating patterns
    // of increasing size and returns the first one that repeats throughout the latter
    // half of the current set of diffs.
    fn try_detect_cycle(&self) -> Option<Vec<usize>> {
        // assume the last half of the data is all within the cycle
        // if that is not the case, we'll have to wait for more data
        let mut sequence = Vec::from(&self.diffs[(self.diffs.len() / 2)..]);
        sequence.reverse();

        for candidate_len in 2..(sequence.len() / 2) {
            let mut candidate = Vec::from(&sequence[0..candidate_len]);
            if sequence
                .chunks_exact(candidate_len)
                .all(|chunk| chunk == candidate)
            {
                candidate.reverse();
                return Some(candidate);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_rock() {
        let mut chamber = Chamber::new();

        chamber.drop_rock(&vec![(0, 0), (0, 1), (1, 0), (1, 1)][..]);
        assert_eq!(
            chamber.current_rock,
            [Some((2, 4)), Some((2, 5)), Some((3, 4)), Some((3, 5)), None]
        );

        chamber.heights = [12, 20, 5, 0, 8, 9, 15];
        chamber.drop_rock(&vec![(0, 0), (0, 1), (1, 0), (1, 1)][..]);
        assert_eq!(
            chamber.current_rock,
            [
                Some((2, 24)),
                Some((2, 25)),
                Some((3, 24)),
                Some((3, 25)),
                None
            ]
        );
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let mut chamber = Chamber::new();

        let gusts: Vec<i8> = input
            .trim()
            .as_bytes()
            .iter()
            .map(|b| *b as i8 - b'=' as i8)
            .collect();

        let rocks = [
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // horizontal line
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // plus
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // backward L
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // vertical line
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],         // square
        ];

        assert_eq!(
            chamber.height_after_rocks_stopped(2022, &gusts[..], &rocks),
            3068
        );
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let mut chamber = Chamber::new();

        let gusts: Vec<i8> = input
            .trim()
            .as_bytes()
            .iter()
            .map(|b| *b as i8 - b'=' as i8)
            .collect();

        let rocks = [
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],         // horizontal line
            vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], // plus
            vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], // backward L
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],         // vertical line
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],         // square
        ];

        assert_eq!(
            chamber.height_after_rocks_stopped(1000000000000, &gusts, &rocks),
            1514285714288
        );
    }
}
