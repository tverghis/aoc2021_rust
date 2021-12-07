const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    let input = INPUT.lines().map(Movement::from).collect::<Vec<_>>();

    let final_position = traverse(Position::start(), &input);

    println!(
        "Part 1: {}",
        final_position.horizontal * final_position.vertical
    );
}

fn traverse(init: Position, moves: &[Movement]) -> Position {
    let mut pos = init;

    for &m in moves {
        pos += m;
    }

    pos
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Position {
    horizontal: u32,
    vertical: u32,
}

impl Position {
    fn start() -> Self {
        Position {
            horizontal: 0,
            vertical: 0,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Movement {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::ops::Add<Movement> for Position {
    type Output = Self;

    fn add(self, rhs: Movement) -> Self::Output {
        match rhs {
            Movement::Forward(n) => Position {
                horizontal: self.horizontal + n,
                ..self
            },
            Movement::Down(n) => Position {
                vertical: self.vertical + n,
                ..self
            },
            Movement::Up(n) => Position {
                vertical: self.vertical.saturating_sub(n),
                ..self
            },
        }
    }
}

impl std::ops::AddAssign<Movement> for Position {
    fn add_assign(&mut self, rhs: Movement) {
        let new_pos = *self + rhs;

        self.horizontal = new_pos.horizontal;
        self.vertical = new_pos.vertical;
    }
}

impl From<&str> for Movement {
    fn from(input: &str) -> Self {
        let parts: [_; 2] = input
            .split_whitespace()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        match &parts {
            ["forward", n] => Movement::Forward(n.parse().unwrap()),
            ["down", n] => Movement::Down(n.parse().unwrap()),
            ["up", n] => Movement::Up(n.parse().unwrap()),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_move_forward() {
        let pos = Position {
            horizontal: 10,
            vertical: 0,
        };
        assert_eq!(
            pos + Movement::Forward(10),
            Position {
                horizontal: 20,
                vertical: 0
            }
        );
    }

    #[test]
    fn test_add_move_down() {
        let pos = Position {
            horizontal: 0,
            vertical: 10,
        };
        assert_eq!(
            pos + Movement::Down(10),
            Position {
                horizontal: 0,
                vertical: 20
            }
        );
    }

    #[test]
    fn test_add_move_up() {
        let pos = Position {
            horizontal: 0,
            vertical: 10,
        };
        assert_eq!(
            pos + Movement::Up(10),
            Position {
                horizontal: 0,
                vertical: 0
            }
        );
        assert_eq!(
            pos + Movement::Up(20),
            Position {
                horizontal: 0,
                vertical: 0
            }
        );
    }

    #[test]
    fn test_movement_parsing() {
        assert_eq!(Movement::from("forward 8"), Movement::Forward(8));
        assert_eq!(Movement::from("down 8"), Movement::Down(8));
        assert_eq!(Movement::from("up 8"), Movement::Up(8));
    }

    #[test]
    fn test_traverse() {
        let init = Position::start();

        assert_eq!(traverse(init, &[]), init);
        assert_eq!(
            traverse(
                init,
                &[
                    Movement::Forward(5),
                    Movement::Down(5),
                    Movement::Forward(8),
                    Movement::Up(3),
                    Movement::Down(8),
                    Movement::Forward(2)
                ]
            ),
            Position {
                horizontal: 15,
                vertical: 10,
            }
        );
    }
}
