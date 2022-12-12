// Advent of Code 2022: Day 11
// https://adventofcode.com/2022/day/11
// Usage: `cargo run <input-file>`

use day_11::Monkey;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let mut monkeys: Vec<_> = input.trim().split("\n\n").map(Monkey::new).collect();

    for round in 0..20 {
        println!("Round {}", round + 1);
        for i in 0..monkeys.len() {
            println!("Monkey {i}");
            while let Some((item, target_monkey)) = monkeys[i].inspect_and_throw() {
                println!("Monkey {i} throwing {item} to {target_monkey}");
                monkeys[target_monkey].catch(item);
            }
        }

        println!(
            "After round {}, the monkeys are holding these items",
            round + 1
        );
        for (i, monkey) in monkeys[..].iter().enumerate() {
            println!("Monkey {i} was holding {:?}", &monkey.items);
        }
    }

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected items {} times", i, monkey.inspections);
    }

    monkeys.sort_by_key(|m| m.inspections);
    let part_1: usize = monkeys
        .iter()
        .rev()
        .take(2)
        .map(|m| m.inspections)
        .product();
    println!("The answer for part 1 is {part_1}");
}
