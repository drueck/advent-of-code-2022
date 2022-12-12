use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub enum Operator {
    Plus,
    Times,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Operand {
    Old,
    Literal(usize),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Operation {
    pub operator: Operator,
    pub operand: Operand,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Monkey {
    number: usize,
    pub items: VecDeque<usize>,
    operation: Operation,
    test_divisor: usize,
    true_monkey: usize,
    false_monkey: usize,
    pub inspections: usize,
}

impl Monkey {
    // :sweat_smile: this would be so much cleaner in elixir!
    pub fn new(s: &str) -> Self {
        let mut lines = s.split('\n');

        let number = (lines.next().unwrap().as_bytes()[7] - b'0') as usize;

        let items = lines
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();

        let mut operation_iter = lines
            .next()
            .unwrap()
            .split("= old ")
            .nth(1)
            .unwrap()
            .split(' ');

        let operator = match operation_iter.next().unwrap() {
            "+" => Operator::Plus,
            "*" => Operator::Times,
            _ => panic!("unsupported operator"),
        };
        let operand = match operation_iter.next().unwrap() {
            "old" => Operand::Old,
            n => Operand::Literal(n.parse().unwrap()),
        };
        let operation = Operation { operator, operand };

        let test_divisor = lines
            .next()
            .unwrap()
            .split("by ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let true_monkey = (lines.next().unwrap().as_bytes().last().unwrap() - b'0') as usize;

        let false_monkey = (lines.next().unwrap().as_bytes().last().unwrap() - b'0') as usize;

        Self {
            number,
            items,
            operation,
            test_divisor,
            true_monkey,
            false_monkey,
            inspections: 0,
        }
    }

    // returns (item, monkey) where the item is the worry level
    // of the first item in the queue after the inspection and
    // your relief factor being applied, and the monkey is which
    // monkey to which to toss the item.
    pub fn inspect_and_throw(&mut self) -> Option<(usize, usize)> {
        let mut item = self.items.pop_front()?;
        self.inspections += 1;

        let operand = match self.operation.operand {
            Operand::Old => item,
            Operand::Literal(n) => n,
        };
        item = match self.operation.operator {
            Operator::Plus => {
                // println!("({item} + {operand}) / 3");
                (item + operand) / 3
            }
            Operator::Times => {
                // println!("{item} * ({operand} / 3)");
                (item * operand) / 3
            }
        };

        let target_monkey = match item % self.test_divisor {
            0 => self.true_monkey,
            _ => self.false_monkey,
        };

        Some((item, target_monkey))
    }

    pub fn catch(&mut self, item: usize) {
        self.items.push_back(item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::fs;

    #[test]
    fn monkey_parsing() {
        let input = fs::read_to_string("test-input.txt").unwrap();
        let monkeys: Vec<_> = input.trim().split("\n\n").map(Monkey::new).collect();

        let expected_monkeys = vec![
            Monkey {
                number: 0,
                items: VecDeque::from([79, 98]),
                operation: Operation {
                    operator: Operator::Times,
                    operand: Operand::Literal(19),
                },
                test_divisor: 23,
                true_monkey: 2,
                false_monkey: 3,
                inspections: 0,
            },
            Monkey {
                number: 1,
                items: VecDeque::from([54, 65, 75, 74]),
                operation: Operation {
                    operator: Operator::Plus,
                    operand: Operand::Literal(6),
                },
                test_divisor: 19,
                true_monkey: 2,
                false_monkey: 0,
                inspections: 0,
            },
            Monkey {
                number: 2,
                items: VecDeque::from([79, 60, 97]),
                operation: Operation {
                    operator: Operator::Times,
                    operand: Operand::Old,
                },
                test_divisor: 13,
                true_monkey: 1,
                false_monkey: 3,
                inspections: 0,
            },
            Monkey {
                number: 3,
                items: VecDeque::from([74]),
                operation: Operation {
                    operator: Operator::Plus,
                    operand: Operand::Literal(3),
                },
                test_divisor: 17,
                true_monkey: 0,
                false_monkey: 1,
                inspections: 0,
            },
        ];

        assert_eq!(monkeys, expected_monkeys);
    }

    #[test]
    fn test_inspect_and_throw() {
        let mut monkey = Monkey {
            number: 0,
            items: VecDeque::from([79, 98]),
            operation: Operation {
                operator: Operator::Times,
                operand: Operand::Literal(19),
            },
            test_divisor: 23,
            true_monkey: 2,
            false_monkey: 3,
            inspections: 0,
        };

        assert_eq!(monkey.inspect_and_throw(), Some((500, 3)));
        assert_eq!(monkey.inspect_and_throw(), Some((620, 3)));
        assert_eq!(monkey.inspect_and_throw(), None);
        assert_eq!(monkey.inspections, 2);
    }
}
