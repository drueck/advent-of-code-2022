// Advent of Code 2022: Day 18
// https://adventofcode.com/2022/day/18
// Usage: `cargo run <input-file>`

use day_18::UnitCube;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input file");
    let mut exposed_faces = HashSet::new();
    let cubes = input.trim().split('\n').map(UnitCube::from_str);

    for cube in cubes {
        for face in cube.faces() {
            if !exposed_faces.insert(face) {
                exposed_faces.remove(&face);
            }
        }
    }

    println!("The answer to part 1 is {}", exposed_faces.len());
}
