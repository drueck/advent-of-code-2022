// Advent of Code 2022: Day 18
// https://adventofcode.com/2022/day/18
// Usage: `cargo run <input-file>`

use day_18::{BoundingCube, UnitCube};
use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(input_filename).expect("failed to read input file");
    let cubes: HashSet<_> = input.trim().split('\n').map(UnitCube::from_str).collect();

    let bounding_cube = BoundingCube::from_cubes(&cubes);
    let mut exposed_faces = HashSet::new();

    for cube in cubes.iter() {
        for face in cube.faces {
            if !exposed_faces.insert(face) {
                exposed_faces.remove(&face);
            }
        }
    }

    println!("The answer to part 1 is {}", exposed_faces.len());

    let mut exterior_faces = HashSet::new();
    let mut queue = VecDeque::new();
    let mut enqueued = HashSet::new();

    let starting_cube = UnitCube::new(
        bounding_cube.min_x,
        bounding_cube.min_y,
        bounding_cube.min_z,
    );

    queue.push_back(starting_cube);

    while let Some(cube) = queue.pop_front() {
        for (i, adjacent) in cube
            .adjacent_cubes_within(&bounding_cube)
            .iter()
            .enumerate()
        {
            if let Some(adjacent_cube) = adjacent {
                if exposed_faces.contains(&cube.faces[i]) {
                    exterior_faces.insert(cube.faces[i]);
                } else {
                    if !enqueued.contains(&adjacent_cube.coords()) {
                        queue.push_back(adjacent_cube.clone());
                        enqueued.insert(adjacent_cube.coords());
                    }
                }
            }
        }
    }

    println!("The answer to part 2 is {}", exterior_faces.len());
}
