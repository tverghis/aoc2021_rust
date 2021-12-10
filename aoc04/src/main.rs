use game_state::GameState;

mod board;
mod game_state;
mod parse;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    let mut game = GameState::new(INPUT);
    game.run();

    println!("Part 1: {}", game.winning_scores.first().unwrap());
    println!("Part 2: {}", game.winning_scores.last().unwrap());
}
