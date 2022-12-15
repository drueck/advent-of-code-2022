// Advent of Code 2022: Day 11
// https://adventofcode.com/2022/day/11
// Usage: `cargo run <input-file>`

use day_11::Monkey;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let mut monkeys_part_1: Vec<_> = input.trim().split("\n\n").map(Monkey::new).collect();
    let mut monkeys_part_2 = monkeys_part_1.clone();

    let common_divisor: usize = monkeys_part_2.iter().map(|m| m.test_divisor).product();

    let part_1 = play_keep_away(&mut monkeys_part_1, 20, &|worry| worry / 3);
    let part_2 = play_keep_away(&mut monkeys_part_2, 10_000, &|worry| worry % common_divisor);

    println!("The answer for part 1 is {part_1}");
    println!("The answer for part 2 is {part_2}");
}

fn play_keep_away<F>(monkeys: &mut [Monkey], rounds: usize, manage_worry: &F) -> usize
where
    F: Fn(usize) -> usize,
{
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((item, target_monkey)) = monkeys[i].inspect_and_throw(manage_worry) {
                monkeys[target_monkey].catch(item);
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspections);
    monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let mut monkeys: Vec<_> = input.trim().split("\n\n").map(Monkey::new).collect();
        assert_eq!(play_keep_away(&mut monkeys, 20, &|worry| worry / 3), 10_605);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").expect("failed to read input file");
        let mut monkeys: Vec<_> = input.trim().split("\n\n").map(Monkey::new).collect();
        let common_divisor: usize = monkeys.iter().map(|m| m.test_divisor).product();

        assert_eq!(
            play_keep_away(&mut monkeys, 10_000, &|worry| worry % common_divisor),
            2_713_310_158
        );
    }
}
