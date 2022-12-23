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
    pub fn x_range(&self, y: isize) -> Option<XRange> {
        match self.range - (self.location.y - y).abs() {
            x_offset if x_offset >= 0 => Some(XRange::new(
                self.location.x - x_offset,
                self.location.x + x_offset,
            )),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct XRange {
    pub min: isize,
    pub max: isize,
}

impl XRange {
    pub fn new(min: isize, max: isize) -> Self {
        assert!(min <= max);
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
        if !self.mergeable(&other) {
            assert!(self.mergeable(&other));
        }

        self.min = min(self.min, other.min);
        self.max = max(self.max, other.max);
    }

    pub fn constrained(&self, constraint: &Self) -> Option<Self> {
        if self.max < constraint.min || self.min > constraint.max {
            return None;
        }

        let mut range = self.clone();
        if range.min < constraint.min && range.max >= constraint.min {
            range.min = constraint.min;
        }
        if range.max > constraint.max && range.min <= constraint.max {
            range.max = constraint.max;
        }
        Some(range)
    }

    pub fn len(&self) -> usize {
        (self.max - self.min).abs() as usize + 1
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
        self.ranges.retain(|range| !new_range.contains(&range));

        let mergeable_ranges: Vec<_> = self
            .ranges
            .drain_filter(|range| range.mergeable(&new_range))
            .collect();

        for range in mergeable_ranges {
            new_range.merge(&range);
        }

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
    fn test_point_manhattan_distance() {
        let sensor_location = Point::new(8, 7);
        let closest_beacon = Point::new(2, 10);
        assert_eq!(sensor_location.manhattan_distance(&closest_beacon), 9);
    }

    #[test]
    fn test_sensor_x_range() {
        let sensor = Sensor::new(Point::new(8, 7), Point::new(2, 10));
        assert_eq!(sensor.range, 9);
        assert_eq!(sensor.x_range(10), Some(XRange::new(2, 14)));
        assert_eq!(sensor.x_range(100), None);
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

    #[test]
    fn test_x_range_constrained() {
        let search_space = XRange::new(0, 20);
        assert_eq!(XRange::new(-20, -1).constrained(&search_space), None);

        assert_eq!(
            XRange::new(-20, 0).constrained(&search_space),
            Some(XRange::new(0, 0))
        );

        assert_eq!(
            XRange::new(-20, 5).constrained(&search_space),
            Some(XRange::new(0, 5))
        );

        assert_eq!(
            XRange::new(5, 25).constrained(&search_space),
            Some(XRange::new(5, 20))
        );

        assert_eq!(XRange::new(21, 25).constrained(&search_space), None);
    }
}
