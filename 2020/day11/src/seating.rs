//! # Seating
//!
//! This module contains the `SeatingSystem` object and its methods. The
//! `SeatingSystem` object models an actual seating system with a grid of
//! seats where:
//!
//! - `'L'` - is an empty seat
//! - `'#'` - is an occupied seat
//! - `'.'` - is the floor
//!

pub struct SeatingSystem {
    grid: Vec<Vec<char>>,
}

impl SeatingSystem {
    /// returns a new instance of `SeatingSystem` given a grid of seats
    /// where '#' is an occupied seat, 'L' is an empty seat, and '.' is the
    /// floor
    pub fn new(grid: &[Vec<char>]) -> SeatingSystem {
        SeatingSystem {
            grid: grid.to_vec(),
        }
    }

    /// updates and applies the rules from pt1, returns a boolean that
    /// indicates if the grid has changed or not
    pub fn update(&mut self) -> bool {
        let mut new_grid: Vec<Vec<char>> = self.grid.to_vec();
        let mut has_changed = false;

        #[allow(clippy::needless_range_loop)]
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                let seat = self.grid[i][j];
                let num_adjacent = self.num_adjacent(i, j);

                // rule 1
                if seat == 'L' && num_adjacent == 0 {
                    new_grid[i][j] = '#';
                    has_changed = true;

                // rule 2
                } else if seat == '#' && num_adjacent >= 4 {
                    new_grid[i][j] = 'L';
                    has_changed = true;
                }
            }
        }

        if has_changed {
            self.grid = new_grid;
        }

        has_changed
    }

    /// updates and applies the rules from pt2, returns a boolean that
    /// indicates if the seating has changed or not
    pub fn update2(&mut self) -> bool {
        let mut new_grid: Vec<Vec<char>> = self.grid.to_vec();
        let mut has_changed = false;

        #[allow(clippy::needless_range_loop)]
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                let seat = self.grid[i][j];
                let num_can_see = self.num_can_see(i, j);

                // rule 1
                if seat == 'L' && num_can_see == 0 {
                    new_grid[i][j] = '#';
                    has_changed = true;

                // rule 2
                } else if seat == '#' && num_can_see >= 5 {
                    new_grid[i][j] = 'L';
                    has_changed = true;
                }
            }
        }

        if has_changed {
            self.grid = new_grid;
        }

        has_changed
    }

    /// returns the number of adjacent occupied seats next to the original
    /// seat (row, col)
    fn num_adjacent(&self, row: usize, col: usize) -> i32 {
        let max_rows = self.grid.len() as i32;
        let max_cols = self.grid[0].len() as i32;

        let mut res = 0;

        for i in -1..=1 {
            for j in -1..=1 {
                let row_num = (row as i32) + i;
                let col_num = (col as i32) + j;

                // only count seats that are 1) in bounds and 2) not the
                // original seat
                if !(i == 0 && j == 0)
                    && (row_num < max_rows)
                    && (row_num >= 0)
                    && (col_num < max_cols)
                    && (col_num >= 0)
                    && (self.grid[row_num as usize][col_num as usize] == '#')
                {
                    res += 1;
                }
            }
        }

        res
    }

    /// returns the number of occupied seats that the current seat (row, col)
    /// can see (in all 8 directions)
    fn num_can_see(&self, row: usize, col: usize) -> i32 {
        let max_rows = self.grid.len() as i32;
        let max_cols = self.grid[0].len() as i32;

        let mut res: i32 = 0;

        let dirs: Vec<(i32, i32)> = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ];

        let mut dirs_seen: Vec<bool> = vec![false; dirs.len()];

        let mut layer = 1;

        loop {
            let seats: Vec<(usize, (i32, i32))> = dirs
                .iter()
                .map(|dir| (row as i32 + layer * dir.0, col as i32 + layer * dir.1))
                .enumerate()
                .filter(|(idx, (row_num, col_num))| {
                    (!dirs_seen[*idx])
                        && (*row_num >= 0)
                        && (*row_num < max_rows)
                        && (*col_num >= 0)
                        && (*col_num < max_cols)
                })
                .collect();

            for (idx, (i, j)) in &seats {
                let seat = self.grid[*i as usize][*j as usize];
                match seat {
                    '#' => {
                        dirs_seen[*idx] = true;
                        res += 1;
                    }
                    'L' => {
                        dirs_seen[*idx] = true;
                    }
                    _ => {}
                };
            }

            if seats.is_empty() {
                break;
            }

            layer += 1;
        }

        res
    }

    /// a helper function that counts the number of occupied seats '#' in the
    /// grid
    pub fn count_seats(&self) -> i32 {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&ch| ch == '#').count() as i32)
            .sum()
    }

    /// helper function that prints the entire grid
    #[allow(dead_code)]
    pub fn print_grid(&self) {
        for row in self.grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_adjacent() {
        let input: Vec<Vec<char>> = vec![
            vec!['L', 'L', '#', '#'],
            vec!['.', '#', '#', '#'],
            vec!['#', '#', '#', '#'],
        ];

        let seating = SeatingSystem::new(&input);

        assert_eq!(seating.num_adjacent(1, 1), 5);
        // assert_eq!(seating.num_adjacent(2, 3), 3);
        // assert_eq!(seating.num_adjacent(0, 0), 1);
        // assert_eq!(seating.num_adjacent(0, 3), 3);
    }

    #[test]
    fn test_num_can_see() {
        let input: Vec<Vec<char>> = vec![
            vec!['.', '.', '.', '#', '#'],
            vec!['.', '.', '.', '#', '#'],
            vec!['L', '#', '.', 'L', '#'],
            vec!['L', '#', '.', '#', 'L'],
            vec!['#', '.', '.', '#', 'L'],
        ];

        let seating = SeatingSystem::new(&input);

        assert_eq!(seating.num_can_see(0, 0), 2);
        assert_eq!(seating.num_can_see(1, 4), 4);
        assert_eq!(seating.num_can_see(2, 2), 4);
    }
}
