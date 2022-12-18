use std::cmp::Ordering;
use Element::{Int, List};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Element {
    List(Vec<Element>),
    Int(usize),
}

impl Element {
    pub fn new(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        match bytes[0] {
            b'[' => {
                // this element is a list
                let mut list = vec![];
                let mut cursor = 0;
                loop {
                    cursor += 1;
                    match bytes[cursor] {
                        b'[' => {
                            // found an inner list within the outer list
                            let start = cursor;
                            let mut nested_depth = 0;
                            for (i, b) in bytes[(start + 1)..].iter().enumerate() {
                                match b {
                                    b']' => {
                                        if nested_depth == 0 {
                                            cursor = start + 1 + i;
                                            break;
                                        }
                                        nested_depth -= 1;
                                    }
                                    b'[' => {
                                        nested_depth += 1;
                                    }
                                    _ => {}
                                }
                            }
                            list.push(Element::from_bytes(&bytes[start..=cursor]));
                        }
                        b']' => {
                            // end of the outer list, so we're done
                            break;
                        }
                        b',' => {
                            // comma separating items in the outer list, continue
                        }
                        n if (b'0'..=b'9').contains(&n) => {
                            // found a number element in the outer list
                            let start = cursor;
                            for (i, b) in bytes[(start + 1)..].iter().enumerate() {
                                if !(b'0'..=b'9').contains(b) {
                                    cursor = start + i;
                                    break;
                                }
                            }
                            list.push(Element::from_bytes(&bytes[start..=cursor]));
                        }
                        n => {
                            panic!("Unexpected character {}!", n as char);
                        }
                    }

                    if cursor == bytes.len() - 1 {
                        break;
                    };
                }
                List(list)
            }
            n if (b'0'..=b'9').contains(&n) => {
                // this element is a number
                let num = bytes
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |result, (j, digit)| {
                        result + (*digit as usize - b'0' as usize) * (10usize.pow(j as u32))
                    });
                Int(num)
            }
            _ => panic!("Not a list or an int!"),
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(s), Int(o)) => s.cmp(o),
            (Int(_), List(_)) => List(vec![self.clone()]).cmp(other),
            (List(_), Int(_)) => self.cmp(&List(vec![other.clone()])),
            (List(s), List(o)) => s.cmp(o),
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Element::{Int, List};

    #[test]
    fn parse_just_int() {
        assert_eq!(Element::new("7"), Int(7));
        assert_eq!(Element::new("23"), Int(23));
        assert_eq!(Element::new("1010"), Int(1010));
    }

    #[test]
    fn parse_list_of_int() {
        assert_eq!(Element::new("[1,2,3]"), List(vec![Int(1), Int(2), Int(3)]))
    }

    #[test]
    fn parse_list_of_lists() {
        assert_eq!(
            Element::new("[[1],[2,3,4]]"),
            List(vec![List(vec![Int(1)]), List(vec![Int(2), Int(3), Int(4)])])
        );
    }

    #[test]
    fn parse_mixed_content() {
        assert_eq!(
            Element::new("[[4,4],4,4]"),
            List(vec![List(vec![Int(4), Int(4)]), Int(4), Int(4)])
        )
    }

    #[test]
    fn parse_deeply_nested_lists() {
        assert_eq!(
            Element::new("[1,[2,[3,[4,[5,6,0]]]],8,9]"),
            List(vec![
                Int(1),
                List(vec![
                    Int(2),
                    List(vec![
                        Int(3),
                        List(vec![Int(4), List(vec![Int(5), Int(6), Int(0)])])
                    ])
                ]),
                Int(8),
                Int(9)
            ])
        )
    }

    #[test]
    fn compare_ints() {
        assert!(Int(1) < Int(2));
        assert!(Int(2) == Int(2));
    }
}
