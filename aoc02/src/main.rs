const INPUT: &str = include_str!("../input.txt");

fn main() {
    let input = INPUT.lines().map(Movement::from).collect::<Vec<_>>();

    let mut sub = Submarine::new();
    sub.traverse_multiple(&input);

    // Part 1's solution was obsoleted by Part 2 :(

    println!("Part 2: {}", sub.pos.horizontal * sub.pos.vertical);
}

struct Submarine {
    pos: Position,
    aim: i32,
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            pos: Position::start(),
            aim: 0,
        }
    }

    fn traverse_multiple(&mut self, moves: &[Movement]) {
        for m in moves {
            self.traverse(m);
        }
    }

    fn traverse(&mut self, movement: &Movement) {
        match movement {
            Movement::Down(n) => self.aim += *n as i32,
            Movement::Up(n) => self.aim -= *n as i32,
            Movement::Forward(n) => {
                self.pos.horizontal += n;
                self.pos.vertical += (self.aim * *n as i32) as u32;
            }
        };
    }
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
    fn test_movement_parsing() {
        assert_eq!(Movement::from("forward 8"), Movement::Forward(8));
        assert_eq!(Movement::from("down 8"), Movement::Down(8));
        assert_eq!(Movement::from("up 8"), Movement::Up(8));
    }

    #[test]
    fn test_traverse() {
        let mut sub = Submarine::new();

        sub.traverse_multiple(&[]);
        assert_eq!(sub.pos, Position::start());

        sub.traverse_multiple(&[
            Movement::Forward(5),
            Movement::Down(5),
            Movement::Forward(8),
            Movement::Up(3),
            Movement::Down(8),
            Movement::Forward(2),
        ]);
        assert_eq!(
            sub.pos,
            Position {
                horizontal: 15,
                vertical: 60
            }
        );
    }
}
