use std::collections::HashMap;
use std::ops::RangeInclusive;

pub struct Cave {
    pub map: HashMap<(usize, usize), char>,
    pub deepest_rocks: HashMap<usize, usize>,
    pub sand_at_rest: usize,
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
            sand_at_rest: 0,
        }
    }

    // drop a grain of sand and return the position at which it rests, or None if it falls forever
    pub fn drop_sand(&mut self, x: usize, y: usize) -> Option<(usize, usize)> {
        // falling forever
        match self.deepest_rocks.get(&x) {
            None => return None,
            Some(deepest_y) => {
                if y >= *deepest_y {
                    return None;
                }
            }
        }

        // down
        if self.map.get(&(x, y + 1)).is_none() {
            self.drop_sand(x, y + 1)
        // down and left
        } else if self.map.get(&(x - 1, y + 1)).is_none() {
            self.drop_sand(x - 1, y + 1)
        // down and right
        } else if self.map.get(&(x + 1, y + 1)).is_none() {
            self.drop_sand(x + 1, y + 1)
        // can't move, so add it to the map
        } else {
            self.sand_at_rest += 1;
            self.map.insert((x, y), 'o');
            Some((x, y))
        }
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
    fn test_drop_sand() {
        let input = std::fs::read_to_string("test-input.txt").unwrap();
        let mut cave = Cave::new(&input);
        cave.drop_sand(500, 0);

        assert_eq!(cave.sand_at_rest, 1);
        assert_eq!(cave.map.get(&(500, 8)), Some(&'o'));
    }

    #[test]
    fn drop_till_you_stop() {
        let input = std::fs::read_to_string("test-input.txt").unwrap();
        let mut cave = Cave::new(&input);

        while cave.drop_sand(500, 0).is_some() {}
        assert_eq!(cave.sand_at_rest, 24);
    }
}
