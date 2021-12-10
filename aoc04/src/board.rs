use std::fmt::Debug;

#[derive(PartialEq, Eq)]
pub struct Board<T: PartialEq>(Vec<Vec<T>>);

impl<T: PartialEq> Board<T> {
    pub fn new(items: Vec<Vec<T>>) -> Self {
        Board(items)
    }

    pub fn rows(&self) -> &[Vec<T>] {
        &self.0
    }

    pub fn find(&self, needle: T) -> Option<(usize, usize)> {
        for (i, row) in self.0.iter().enumerate() {
            for (j, item) in row.iter().enumerate() {
                if needle == *item {
                    return Some((i, j));
                }
            }
        }

        None
    }

    pub fn set(&mut self, row: usize, col: usize, val: T) {
        self.0[row][col] = val;
    }
}

impl Board<bool> {
    pub fn is_winner(&self) -> bool {
        // check rows
        if self.0.iter().any(|row| row.iter().all(|i| *i)) {
            return true;
        }

        // check columns
        let mut has_winning_col = false;

        for i in 0..5 {
            let mut is_winning_col = true;
            for row in self.0.iter() {
                is_winning_col &= row[i];
            }

            has_winning_col |= is_winning_col;
        }

        has_winning_col
    }

    pub fn has_marked(&self, row: usize, col: usize) -> bool {
        self.0[row][col]
    }
}

impl<T> Debug for Board<T>
where
    T: PartialEq,
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.0.iter() {
            writeln!(f, "{:?}", row)?;
        }

        Ok(())
    }
}

pub fn create_virtual_board() -> Board<bool> {
    let items = vec![vec![false; 5]; 5];

    Board::new(items)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_win_column() {
        let board = Board::<bool>::new(vec![
            vec![false, false, true, true, false],
            vec![true, false, true, true, true],
            vec![false, false, true, false, true],
            vec![false, true, true, true, true],
            vec![true, true, true, false, false],
        ]);

        assert_eq!(board.is_winner(), true);
    }
}
