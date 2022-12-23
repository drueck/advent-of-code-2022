// Advent of Code 2022: Day 15
// https://adventofcode.com/2022/day/15
// Usage: `cargo run <input-file>`

use day_15::{Coverage, Point, Sensor, XRange};
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let input = fs::read_to_string(&input_filename).expect("failed to read input file");
    let sensors = parse_input(&input);

    let (y, search_space) = match &input_filename[..] {
        "test-input.txt" => (10, XRange::new(0, 20)),
        "input.txt" => (2_000_000, XRange::new(0, 4_000_000)),
        _ => panic!("Please use either input.txt or test-input.txt"),
    };

    println!(
        "The number of locations on the given y that cannot contain a beacon are: {}",
        part_1(&sensors, y)
    );

    println!(
        "The tuning frequency for the missing beacon is: {}",
        part_2(&sensors, &search_space).expect("couldn't find the missing beacon!")
    );
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let d = "(-?[0-9]+)";
    let format = format!(r"Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}");
    let re = Regex::new(&format[..]).unwrap();

    input
        .trim()
        .split('\n')
        .map(|line| {
            let numbers = re.captures(line).expect("didn't match the regex");
            let sensor = Point::new(numbers[1].parse().unwrap(), numbers[2].parse().unwrap());
            let beacon = Point::new(numbers[3].parse().unwrap(), numbers[4].parse().unwrap());
            Sensor::new(sensor, beacon)
        })
        .collect()
}

fn part_1(sensors: &[Sensor], y: isize) -> usize {
    let mut coverage = Coverage::new();
    for x_range in sensors.iter().flat_map(|sensor| sensor.x_range(y)) {
        coverage.add_range(x_range);
    }

    let beacon_locations_in_row: HashSet<isize> = sensors
        .iter()
        .filter(|sensor| sensor.closest_beacon.y == y)
        .map(|sensor| sensor.closest_beacon.x)
        .collect();

    coverage.len() - beacon_locations_in_row.len()
}

fn part_2(sensors: &[Sensor], search_space: &XRange) -> Option<isize> {
    for y in search_space.min..=search_space.max {
        let mut coverage = Coverage::new();
        for x_range in sensors
            .iter()
            .flat_map(|sensor| sensor.x_range(y)?.constrained(&search_space))
        {
            coverage.add_range(x_range);
        }

        if coverage.len() < search_space.len() {
            let mut ranges: Vec<_> = coverage.ranges.iter().collect();
            let x = match ranges.len() {
                1 => {
                    if ranges[0].min == search_space.min + 1 {
                        search_space.min
                    } else {
                        search_space.max
                    }
                }
                2 => {
                    ranges.sort();
                    ranges[0].max + 1
                }
                n => panic!("Expected either one or two ranges, got {n}"),
            };
            return Some(x * 4_000_000 + y);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let sensors = parse_input(&input);

        assert_eq!(sensors.len(), 14);
        assert_eq!(
            sensors[0],
            Sensor::new(Point::new(2, 18), Point::new(-2, 15))
        );
        assert_eq!(
            sensors[13],
            Sensor::new(Point::new(20, 1), Point::new(15, 3))
        );
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let sensors = parse_input(&input);
        assert_eq!(part_1(&sensors, 10), 26);
    }

    #[test]
    fn test_part_2() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let sensors = &parse_input(&input)[..];
        assert_eq!(part_2(&sensors, &XRange::new(0, 20)), Some(56_000_011));
    }
}
