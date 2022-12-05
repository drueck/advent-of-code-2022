#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub quantity: usize,
    pub from: usize,
    pub to: usize,
}

impl Move {
    pub fn new(description: &str) -> Self {
        let mut words = description.split(' ');

        let mut get_next_int = || {
            words
                .nth(1)
                .expect("invalid move")
                .parse()
                .expect("invalid int")
        };

        Move {
            quantity: get_next_int(),
            from: get_next_int(),
            to: get_next_int(),
        }
    }
}

#[test]
fn new_move() {
    assert_eq!(
        Move::new("move 2 from 1 to 3"),
        Move {
            quantity: 2,
            from: 1,
            to: 3
        }
    );
}
