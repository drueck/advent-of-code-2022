#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub quantity: usize,
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn new(quantity: usize, from: usize, to: usize) -> Self {
        Move { quantity, from, to }
    }

    pub fn parse(description: &str) -> Self {
        let mut words = description.split(' ');

        let mut get_next_int = || {
            words
                .nth(1)
                .expect("invalid move")
                .parse()
                .expect("invalid int")
        };

        Move::new(get_next_int(), get_next_int(), get_next_int())
    }
}

pub fn apply_moves_9000(stacks: &mut [Vec<char>], moves: &Vec<Move>) {
    for moov in moves {
        for _ in 0..moov.quantity {
            let krate = stacks[moov.from - 1].pop().expect("invalid move");
            stacks[moov.to - 1].push(krate)
        }
    }
}

pub fn apply_moves_9001(stacks: &mut [Vec<char>], moves: &Vec<Move>) {
    for moov in moves {
        let from_len = stacks[moov.from - 1].len();

        // I benchmarked this commented option which allocates an intermediate Vec.
        // Criterion reported that it was about 300% slower than what I went with.
        // I mainly wanted to get some experience with benchmarking; I'm not 100%
        // sure I set everything up correctly, but the results make sense to me.
        //
        // let mut crates = stacks[moov.from - 1].split_off(from_len - moov.quantity);
        // stacks[moov.to - 1].append(&mut crates);

        for i in (from_len - moov.quantity)..from_len {
            stacks[moov.to - 1].push(stacks[moov.from - 1][i]);
        }
        stacks[moov.from - 1].truncate(from_len - moov.quantity);
    }
}

pub fn top_of_stacks(stacks: &[Vec<char>]) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut parts = input.trim_end().split("\n\n");

    (
        parse_stacks(parts.next().expect("missing stacks descriptions")),
        parse_moves(parts.next().expect("missing moves descriptions")),
    )
}

fn parse_stacks(stacks_descriptions: &str) -> Vec<Vec<char>> {
    let mut lines = stacks_descriptions.split('\n').rev();
    let stack_numbers = lines
        .next()
        .expect("invalid stacks description")
        .trim()
        .as_bytes();

    // we expect at most 9 stacks
    let num_stacks = (stack_numbers.last().unwrap() - b'0') as usize;
    let mut stacks = vec![vec![]; num_stacks];

    for line in lines {
        let bytes = line.as_bytes();
        for (i, stack) in stacks.iter_mut().enumerate() {
            match bytes.get(1 + (i * 4)) {
                Some(byte) if *byte != b' ' => stack.push(*byte as char),
                _ => continue,
            }
        }
    }

    stacks
}

fn parse_moves(move_descriptions: &str) -> Vec<Move> {
    move_descriptions.split('\n').map(Move::parse).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn new_move() {
        assert_eq!(Move::parse("move 2 from 1 to 3"), Move::new(2, 1, 3));
    }

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input");
        let (stacks, moves) = parse_input(&input);

        let expected_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        let expected_moves = vec![
            Move::new(1, 2, 1),
            Move::new(3, 1, 3),
            Move::new(2, 2, 1),
            Move::new(1, 1, 2),
        ];

        assert_eq!(stacks, expected_stacks);
        assert_eq!(moves, expected_moves);
    }

    #[test]
    fn crate_mover_9000() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input");
        let (mut stacks, moves) = parse_input(&input);
        apply_moves_9000(&mut stacks, &moves);
        assert_eq!(top_of_stacks(&stacks), "CMZ");
    }

    #[test]
    fn crate_mover_9001() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input");
        let (mut stacks, moves) = parse_input(&input);
        apply_moves_9001(&mut stacks, &moves);
        assert_eq!(top_of_stacks(&stacks), "MCD");
    }
}
