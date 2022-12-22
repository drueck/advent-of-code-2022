use std::ops::RangeInclusive;

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
    pub fn x_range(&self, y: isize) -> RangeInclusive<isize> {
        let x_offset = self.range - (self.location.y - y).abs();
        (self.location.x - x_offset)..=(self.location.x + x_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_range() {
        let sensor = Sensor::new(Point::new(8, 7), Point::new(2, 10));
        assert_eq!(sensor.range, 9);
        assert_eq!(sensor.x_range(10), 2..=14);
    }
}
