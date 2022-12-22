// Advent of Code 2022: Day 15
// https://adventofcode.com/2022/day/15
// Usage: `cargo run <input-file>`

use day_15::{Point, Sensor};
use regex::Regex;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() {
    let input_filename = env::args().nth(1).expect("please supply an input filename");
    let y = env::args()
        .nth(2)
        .expect("Please specify the y coordinate to check")
        .parse()
        .expect("Invalid y coordinate");
    let input = fs::read_to_string(input_filename).expect("failed to read input file");
    let sensors = parse_input(&input);

    println!(
        "The number of locations on the given y that cannot contain a beacon are: {}",
        part_1(&sensors, y)
    )
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
    let mut xs = HashSet::new();
    for sensor in sensors {
        for x in sensor.x_range(y) {
            xs.insert(x);
        }
    }
    for sensor in sensors.iter().filter(|sensor| sensor.closest_beacon.y == y) {
        xs.remove(&sensor.closest_beacon.x);
    }
    xs.len()
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
    fn test_manhattan_distance() {
        let sensor_location = Point::new(8, 7);
        let closest_beacon = Point::new(2, 10);
        assert_eq!(sensor_location.manhattan_distance(&closest_beacon), 9);
    }

    #[test]
    fn test_part_1() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let sensors = parse_input(&input);
        assert_eq!(part_1(&sensors, 10), 26);
    }
}
