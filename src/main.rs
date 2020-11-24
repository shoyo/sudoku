/// Copyright (c) 2020, Shoyo Inokuchi
///
/// A simple sudoku solver written in Rust.
/// Feel free to to refer to the repository at: https://github.com/shoyo/sudoku for more
/// information.
///
use std::fmt::{Display, Error, Formatter};

mod boards;

const ROWS: usize = 9;
const COLS: usize = 9;
const CAGE_ROWS: usize = 3;
const CAGE_COLS: usize = 3;

struct Sudoku {
    board: Vec<Vec<Option<u8>>>,
}

impl Sudoku {
    /// Intialize a sudoku board.
    /// Takes in an initial board state defined as a vector of tuples.
    /// Tuples take the form of (row index, column index, value). Values should be an
    /// integer between 1 and 9.
    fn new(initial: Vec<(usize, usize, u8)>) -> Result<Self, String> {
        let mut board = Vec::with_capacity(ROWS);
        for _ in 0..ROWS {
            let mut row = Vec::with_capacity(COLS);
            for _ in 0..COLS {
                row.push(None);
            }
            board.push(row);
        }

        for (row, col, val) in initial {
            if row >= ROWS || col >= COLS || val == 0 || val > 9 {
                return Err(format!(
                    "Value: {} at position ({}, {}) is invalid.",
                    val, row, col
                ));
            }
            if board[row][col] != None {
                return Err(format!(
                    "Value already exists at position ({}, {}).",
                    row, col
                ));
            }
            board[row][col] = Some(val);
        }

        Ok(Self { board: board })
    }

    /// Solve the sudoku board with backtracking and return an Ok if successful.
    /// If the board cannot be solved, return an Error.
    /// This function mutates the internal board representation in-place.
    fn solve(&mut self) -> Result<(), ()> {
        let (row, col) = match self.find_open_cell_() {
            Some(cell) => cell,
            None => return Ok(()),
        };
        for val in 1..10 {
            if self.valid_insert(row, col, val) {
                self.board[row][col] = Some(val);
                match self.solve() {
                    Ok(_) => return Ok(()),
                    Err(_) => self.board[row][col] = None,
                }
            }
        }
        Err(())
    }

    /// Return true iff the board is complete and correct.
    fn verify(&self) -> bool {
        for i in 0..ROWS {
            if !self.verify_row_(i) {
                return false;
            }
        }
        for j in 0..COLS {
            if !self.verify_col_(j) {
                return false;
            }
        }
        for ci in 0..CAGE_ROWS {
            for cj in 0..CAGE_COLS {
                if !self.verify_cage_(ci, cj) {
                    return false;
                }
            }
        }
        true
    }

    /// Return true iff the given row on the board is complete and correct.
    fn verify_row_(&self, row: usize) -> bool {
        let mut seen = [false; 10];
        for col in 0..COLS {
            let val = match self.board[row][col] {
                Some(val) => val as usize,
                None => return false,
            };
            if seen[val] || val > 9 {
                return false;
            }
            seen[val] = true;
        }
        true
    }

    /// Return true iff the given column on the board is complete and correct.
    fn verify_col_(&self, col: usize) -> bool {
        let mut seen = [false; 10];
        for row in 0..ROWS {
            let val = match self.board[row][col] {
                Some(val) => val as usize,
                None => return false,
            };
            if seen[val] || val > 9 {
                return false;
            }
            seen[val] = true;
        }
        true
    }

    /// Return true iff the given cage on the board is complete and correct.
    /// A cage refers to a 3-by-3 square on the board with the sudoku constraint.
    fn verify_cage_(&self, cage_row: usize, cage_col: usize) -> bool {
        let mut seen = [false; 10];
        for i in 0..CAGE_ROWS {
            for j in 0..CAGE_COLS {
                let val = match self.board[cage_row * CAGE_ROWS + i][cage_col * CAGE_COLS + j] {
                    Some(val) => val as usize,
                    None => return false,
                };
                if seen[val] || val > 9 {
                    return false;
                }
                seen[val] = true;
            }
        }
        true
    }

    /// Return the row and column indexes for a cell that does not contain a value.
    /// If all cells are filled, return None.
    fn find_open_cell_(&self) -> Option<(usize, usize)> {
        for i in 0..ROWS {
            for j in 0..COLS {
                if self.board[i][j] == None {
                    return Some((i, j));
                }
            }
        }
        None
    }

    /// Return true iff the given value can be placed in the given cell.
    fn valid_insert(&self, row: usize, col: usize, val: u8) -> bool {
        self.board[row][col] == None
            && self.valid_row_insert_(row, val)
            && self.valid_col_insert_(col, val)
            && self.valid_cage_insert_(row / CAGE_ROWS, col / CAGE_COLS, val)
    }

    /// Return true iff the given value can be placed in the given row.
    fn valid_row_insert_(&self, row: usize, val: u8) -> bool {
        for col in 0..COLS {
            if let Some(v) = self.board[row][col] {
                if v == val {
                    return false;
                }
            }
        }
        true
    }

    /// Return true iff the given value can be placed in the given column.
    fn valid_col_insert_(&self, col: usize, val: u8) -> bool {
        for row in 0..ROWS {
            if let Some(v) = self.board[row][col] {
                if v == val {
                    return false;
                }
            }
        }
        true
    }

    /// Return true iff the given value can be placed in the given cage.
    /// A cage refers to a 3-by-3 square on the board with the sudoku constraint.
    fn valid_cage_insert_(&self, cage_row: usize, cage_col: usize, val: u8) -> bool {
        for i in 0..CAGE_ROWS {
            for j in 0..CAGE_COLS {
                if let Some(v) = self.board[cage_row * CAGE_ROWS + i][cage_col * CAGE_COLS + j] {
                    if v == val {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl Display for Sudoku {
    /// Define how the board is formatted when printed.
    fn fmt(&self, _fmt: &mut Formatter<'_>) -> Result<(), Error> {
        for i in 0..ROWS {
            for j in 0..COLS {
                match self.board[i][j] {
                    Some(num) => print!(" {} ", num),
                    None => print!(" - "),
                }
            }
            println!();
        }
        Ok(())
    }
}

/// Example usage of Sudoku API.
fn main() {
    // Initialize puzzle
    let board = boards::VALID_PUZZLE_1.to_vec();
    let mut puzzle = Sudoku::new(board).unwrap();

    println!("BEFORE:");
    println!("{}", puzzle);

    // Solve puzzle
    match puzzle.solve() {
        Ok(_) => {
            println!("AFTER:");
            println!("{}", puzzle);
        }
        Err(_) => {
            println!("Invalid puzzle.");
        }
    }
}

/// Unit tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_puzzle() {
        let puzzle = Sudoku::new(vec![(0, 1, 3), (5, 3, 8), (8, 8, 4)]);
        assert!(puzzle.is_ok());
    }

    #[test]
    fn create_invalid_puzzle() {
        let puzzle = Sudoku::new(vec![(0, 0, 10)]);
        assert!(puzzle.is_err());
    }

    #[test]
    fn verify_valid_solution() {
        let puzzle = Sudoku::new(boards::VALID_SOLUTION.to_vec()).unwrap();
        assert_eq!(puzzle.verify(), true);
    }

    #[test]
    fn verify_invalid_solution() {
        let puzzle = Sudoku::new(boards::INVALID_SOLUTION.to_vec()).unwrap();
        assert_eq!(puzzle.verify(), false);
    }

    #[test]
    fn verify_valid_row() {
        let puzzle = Sudoku::new(boards::VALID_ROW.to_vec()).unwrap();
        assert_eq!(puzzle.verify_row_(4), true);
    }

    #[test]
    fn verify_invalid_row() {
        let puzzle = Sudoku::new(boards::INVALID_ROW.to_vec()).unwrap();
        assert_eq!(puzzle.verify_row_(4), false);
    }

    #[test]
    fn verify_valid_col() {
        let puzzle = Sudoku::new(boards::VALID_COL.to_vec()).unwrap();
        assert_eq!(puzzle.verify_col_(4), true);
    }

    #[test]
    fn verify_invalid_col() {
        let puzzle = Sudoku::new(boards::INVALID_COL.to_vec()).unwrap();
        assert_eq!(puzzle.verify_col_(4), false);
    }

    #[test]
    fn verify_valid_cage() {
        let puzzle = Sudoku::new(boards::VALID_CAGE.to_vec()).unwrap();
        assert_eq!(puzzle.verify_cage_(0, 0), true);
    }

    #[test]
    fn verify_invalid_cage() {
        let puzzle = Sudoku::new(boards::INVALID_CAGE.to_vec()).unwrap();
        assert_eq!(puzzle.verify_cage_(0, 0), false);
    }

    #[test]
    fn try_valid_row_insert() {
        let puzzle = Sudoku::new(Vec::new()).unwrap();
        assert_eq!(puzzle.valid_row_insert_(0, 1), true);
    }

    #[test]
    fn try_invalid_row_insert() {
        let puzzle = Sudoku::new(boards::VALID_ROW.to_vec()).unwrap();
        assert_eq!(puzzle.valid_row_insert_(4, 1), false);
    }

    #[test]
    fn try_valid_col_insert() {
        let puzzle = Sudoku::new(Vec::new()).unwrap();
        assert_eq!(puzzle.valid_col_insert_(0, 1), true);
    }

    #[test]
    fn try_invalid_col_insert() {
        let puzzle = Sudoku::new(boards::VALID_COL.to_vec()).unwrap();
        assert_eq!(puzzle.valid_col_insert_(4, 1), false);
    }

    #[test]
    fn try_valid_cage_insert() {
        let puzzle = Sudoku::new(Vec::new()).unwrap();
        assert_eq!(puzzle.valid_cage_insert_(0, 0, 1), true);
    }

    #[test]
    fn try_invalid_cage_insert() {
        let puzzle = Sudoku::new(boards::VALID_CAGE.to_vec()).unwrap();
        assert_eq!(puzzle.valid_cage_insert_(0, 0, 1), false);
    }

    #[test]
    fn solve_valid_puzzle_1() {
        let mut puzzle = Sudoku::new(boards::VALID_PUZZLE_1.to_vec()).unwrap();
        assert_eq!(puzzle.verify(), false);
        let _ = puzzle.solve();
        assert_eq!(puzzle.verify(), true);
    }

    #[test]
    fn solve_valid_puzzle_2() {
        let mut puzzle = Sudoku::new(boards::VALID_PUZZLE_2.to_vec()).unwrap();
        assert_eq!(puzzle.verify(), false);
        let _ = puzzle.solve();
        assert_eq!(puzzle.verify(), true);
    }

    #[test]
    fn solve_valid_puzzle_3() {
        let mut puzzle = Sudoku::new(boards::VALID_PUZZLE_3.to_vec()).unwrap();
        assert_eq!(puzzle.verify(), false);
        let _ = puzzle.solve();
        assert_eq!(puzzle.verify(), true);
    }
}
