use game_state::GameState;

mod board;
mod game_state;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut game = GameState::new(INPUT);

    let score = game.run();

    println!("Part 1: {}", score);
}
