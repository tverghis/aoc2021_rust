use std::collections::HashSet;

use crate::{
    board::{create_virtual_board, Board},
    parse::parse_draws_and_boards,
};

pub struct GameState {
    pub draws: Vec<u32>,
    pub boards: Vec<Board<u32>>,
    pub virtual_boards: Vec<Board<bool>>,
    pub completed_boards: HashSet<usize>,
    pub winning_scores: Vec<u32>,
}

impl GameState {
    pub fn new(input: &str) -> Self {
        let (_, (draws, boards)) = parse_draws_and_boards(input).unwrap();

        let mut virtual_boards = Vec::with_capacity(boards.len());

        for _ in 0..boards.len() {
            virtual_boards.push(create_virtual_board());
        }

        GameState {
            draws,
            boards,
            virtual_boards,
            completed_boards: HashSet::new(),
            winning_scores: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        for drawn in self.draws.iter() {
            for (b, board) in self.boards.iter().enumerate() {
                if let Some((i, j)) = board.find(*drawn) {
                    self.virtual_boards[b].set(i, j, true);

                    if !self.completed_boards.contains(&b) && self.virtual_boards[b].is_winner() {
                        self.completed_boards.insert(b);
                        self.winning_scores.push(self.score_for_board(b) * drawn);

                        if self.completed_boards.len() == self.boards.len() {
                            // Stop as soon as the last board has won
                            break;
                        }
                    }
                }
            }
        }
    }

    fn score_for_board(&self, board_num: usize) -> u32 {
        let board = &self.boards[board_num];
        let virtual_board = &self.virtual_boards[board_num];

        let mut sum = 0;

        for (i, row) in board.rows().iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if !virtual_board.has_marked(i, j) {
                    sum += *item
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run() {
        let mut game = GameState::new(
            r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"#,
        );

        game.run();

        assert_eq!(game.winning_scores.first(), Some(&4512));
        assert_eq!(game.winning_scores.last(), Some(&1924));
    }
}
