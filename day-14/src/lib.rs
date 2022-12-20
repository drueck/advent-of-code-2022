use std::collections::HashMap;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Cave {
    pub map: HashMap<(usize, usize), char>,
    pub deepest_rocks: HashMap<usize, usize>,
    pub floor: Option<usize>,
    pub grains_at_rest: usize,
}

impl Cave {
    pub fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        let mut deepest_rocks = HashMap::new();

        let mut add_rock = |x, y| {
            map.insert((x, y), '#');
            deepest_rocks
                .entry(x)
                .and_modify(|deepest_y| {
                    if y > *deepest_y {
                        *deepest_y = y;
                    }
                })
                .or_insert(y);
        };

        for line in input.trim().split('\n') {
            let mut segments = line.split(" -> ").map(|segment| {
                let (x_str, y_str) = segment.split_once(',').unwrap();
                (x_str.parse().unwrap(), y_str.parse().unwrap())
            });

            let (mut start_x, mut start_y) = segments.next().unwrap();
            for (end_x, end_y) in segments {
                if start_x == end_x {
                    for y in smart_range(start_y, end_y) {
                        add_rock(start_x, y);
                    }
                } else {
                    for x in smart_range(start_x, end_x) {
                        add_rock(x, start_y);
                    }
                }
                (start_x, start_y) = (end_x, end_y);
            }
        }

        Self {
            map,
            deepest_rocks,
            grains_at_rest: 0,
            floor: None,
        }
    }

    pub fn build_floor(&mut self) {
        self.floor = Some(self.deepest_rocks.values().max().unwrap() + 2)
    }

    // drop a grain of sand and return the position at which it rests,
    // or None if it falls forever or if the cave is totally full
    pub fn drop_sand(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        // cave is full
        if self.map.get(&(x, y)).is_some() {
            return None;
        }

        // falling infinitely
        if self.floor.is_none() {
            match self.deepest_rocks.get(&x) {
                None => return None,
                Some(deepest_y) => {
                    if y >= *deepest_y {
                        return None;
                    }
                }
            }
        }

        if self.is_open(x, y + 1) {
            self.drop_sand(x, y + 1)
        } else if self.is_open(x - 1, y + 1) {
            self.drop_sand(x - 1, y + 1)
        } else if self.is_open(x + 1, y + 1) {
            self.drop_sand(x + 1, y + 1)
        } else {
            self.grains_at_rest += 1;
            self.map.insert((x, y), 'o');
            Some((x, y))
        }
    }

    pub fn is_open(&self, x: usize, y: usize) -> bool {
        let at_the_floor = match self.floor {
            Some(floor) => y == floor,
            None => false,
        };

        !at_the_floor && self.map.get(&(x, y)).is_none()
    }
}

fn smart_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_sand_no_floor() {
        let input = std::fs::read_to_string("test-input.txt").unwrap();
        let mut cave = Cave::new(&input);
        cave.drop_sand(500, 0);

        assert_eq!(cave.grains_at_rest, 1);
        assert_eq!(cave.map.get(&(500, 8)), Some(&'o'));
    }

    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("test-input.txt").unwrap();
        let mut cave = Cave::new(&input);

        while cave.drop_sand(500, 0).is_some() {}
        assert_eq!(cave.grains_at_rest, 24);
    }

    #[test]
    fn part_2() {
        let input = std::fs::read_to_string("test-input.txt").unwrap();
        let mut cave_with_floor = Cave::new(&input);

        cave_with_floor.build_floor();
        assert_eq!(cave_with_floor.floor, Some(11));

        while cave_with_floor.drop_sand(500, 0) != Some((500, 0)) {}
        assert_eq!(cave_with_floor.grains_at_rest, 93);
    }
}
