use std::collections::{HashMap, HashSet};

type Coordinate = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    /// The number on the board mapped to the coordinate location. For these
    /// boards, (0, 0) is the top left corner and (4, 4) is the bottom right.
    pub cells: HashMap<usize, Coordinate>,

    /// A set of the coordinates of the numbers that have been marked using
    /// the `mark_cell` method.
    pub marked_cells: HashSet<Coordinate>,

    /// Self-explanatory.
    pub has_bingo: bool,
}

impl Board {
    /// Creates a new, empty board with no cells or marked cells
    pub fn new() -> Board {
        Board {
            cells: HashMap::new(),
            marked_cells: HashSet::new(),
            has_bingo: false,
        }
    }

    /// Creates a new board and populates the cells given a 5x5 string with
    /// numbers delimited by spaces.
    ///
    /// # Example
    ///
    /// ```rust
    /// let s = " 1  2  3  4  5\n 6  7  8  9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25";
    /// let board = Board::from_string(s);
    /// ```
    pub fn from_string(s: &str) -> Board {
        let mut board = Board::new();

        // each line is a row
        s.lines().enumerate().for_each(|(row, line)| {
            // for each number in the col
            line.split_whitespace()
                .enumerate()
                // insert into the board cells
                .for_each(|(col, marker)| {
                    let num = marker.parse::<usize>().unwrap();
                    board.cells.insert(num, (row, col));
                })
        });

        board
    }

    /// If the board conatins the number, add the coordinate of that number to
    /// the set of marked cells.
    ///
    /// **NOTE: This function automatically sets the has_bingo flag if the**
    /// **board has a bingo.**
    pub fn mark_cell(&mut self, num: usize) {
        if let Some(cell) = self.cells.get(&num) {
            self.marked_cells.insert(*cell);
            self.has_bingo = self.has_bingo();
        }
    }

    /// Returns true if the board contains a bingo
    pub fn has_bingo(&self) -> bool {
        let marked_cells = &self.marked_cells;

        if marked_cells.len() < 5 {
            return false;
        }

        // check rows
        for row in 0..5 {
            if (0..5).all(|col| marked_cells.contains(&(row, col))) {
                return true;
            }
        }

        // check cols
        for col in 0..5 {
            if (0..5).all(|row| marked_cells.contains(&(row, col))) {
                return true;
            }
        }

        // check diagonals
        if (0..5).all(|i| marked_cells.contains(&(i, i))) {
            return true;
        }

        if (0..5).all(|i| marked_cells.contains(&(i, 4 - i))) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_from_string() {
        let s = " 1  2  3  4  5\n 6  7  8  9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25";

        let cells: HashMap<usize, Coordinate> = (0..25).map(|n| (n + 1, (n / 5, n % 5))).collect();

        let board = Board {
            cells,
            marked_cells: HashSet::new(),
            has_bingo: false,
        };

        assert_eq!(Board::from_string(s), board);
    }

    #[test]
    fn test_board_has_bingo() {
        let s = " 1  2  3  4  5\n 6  7  8  9 10\n11 12 13 14 15\n16 17 18 19 20\n21 22 23 24 25";
        let mut board = Board::from_string(s);

        // mark all nums in top row
        for i in 1..6 {
            board.mark_cell(i);
        }

        assert!(board.has_bingo);
    }
}
