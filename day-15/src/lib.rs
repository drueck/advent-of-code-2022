#![feature(hash_drain_filter)]

use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Sensor {
    pub location: Point,
    pub closest_beacon: Point,
    pub range: isize,
}

impl Sensor {
    pub fn new(location: Point, closest_beacon: Point) -> Self {
        let range = location.manhattan_distance(&closest_beacon);
        Self {
            location,
            closest_beacon,
            range,
        }
    }

    // what range of x values for the given y can this sensor see?
    pub fn x_range(&self, y: isize) -> XRange {
        let x_offset = self.range - (self.location.y - y).abs();
        XRange::new(self.location.x - x_offset, self.location.x + x_offset + 1)
        // (self.location.x - x_offset)..(self.location.x + x_offset + 1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct XRange {
    pub min: isize,
    pub max: isize,
}

impl XRange {
    pub fn new(min: isize, max: isize) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, other: &Self) -> bool {
        other.min >= self.min && other.max <= self.max
    }

    pub fn mergeable(&self, other: &Self) -> bool {
        self.intersects(&other) || self.adjacent_to(&other)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        (self.min >= other.min && self.min <= other.max)
            || (other.min >= self.min && other.min <= self.max)
    }

    pub fn adjacent_to(&self, other: &Self) -> bool {
        self.max + 1 == other.min || other.max + 1 == self.min
    }

    pub fn merge(&mut self, other: &Self) {
        assert!(self.mergeable(&other));

        self.min = min(self.min, other.min);
        self.max = max(self.max, other.max);
    }

    pub fn len(&self) -> usize {
        (self.max - self.min).abs() as usize
    }
}

#[derive(Debug)]
pub struct Coverage {
    pub ranges: HashSet<XRange>,
}

impl Coverage {
    pub fn new() -> Self {
        Self {
            ranges: HashSet::new(),
        }
    }

    pub fn add_range(&mut self, mut new_range: XRange) {
        // if the new range contains any existing range, remove the existing range
        self.ranges.retain(|range| !new_range.contains(&range));

        // remove any intersecting ranges
        let mergeable_ranges: Vec<_> = self
            .ranges
            .drain_filter(|range| range.mergeable(&new_range))
            .collect();

        // merge them with the new range
        for range in mergeable_ranges {
            new_range.merge(&range);
        }

        // add the new range
        self.ranges.insert(new_range);
    }

    pub fn len(&self) -> usize {
        self.ranges.iter().map(|r| r.len()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_range() {
        let sensor = Sensor::new(Point::new(8, 7), Point::new(2, 10));
        assert_eq!(sensor.range, 9);
        assert_eq!(sensor.x_range(10), XRange::new(2, 15));
    }

    #[test]
    fn test_x_range_intersects() {
        assert!(XRange::new(1, 10).intersects(&XRange::new(5, 15)));
        assert!(!XRange::new(1, 10).intersects(&XRange::new(11, 12)));
    }

    #[test]
    fn test_x_range_adjacent_to() {
        assert!(XRange::new(-5, -3).adjacent_to(&XRange::new(-2, 5)));
        assert!(!XRange::new(-5, -3).adjacent_to(&XRange::new(-1, 5)));
    }

    #[test]
    fn test_x_range_mergeable() {
        assert!(XRange::new(-5, -1).mergeable(&XRange::new(-2, 10)));
        assert!(!XRange::new(-5, -1).mergeable(&XRange::new(-100, -7)));
    }

    #[test]
    fn test_x_range_merge() {
        let mut range = XRange::new(3, 50);

        range.merge(&XRange::new(-14, 20));
        assert_eq!(range, XRange::new(-14, 50));

        range.merge(&XRange::new(-50, -15));
        assert_eq!(range, XRange::new(-50, 50));
    }
}
