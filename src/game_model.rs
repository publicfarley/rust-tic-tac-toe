#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    X,
    O,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
    Empty,
    Owned(Piece),
}

impl Default for CellState {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameBoard {
    pub cells: [[CellState; 3]; 3],
}

pub struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

impl GameBoard {
    pub const TOP_LEFT: Coordinate = Coordinate { row: 0, col: 0 };
    pub const TOP_CENTER: Coordinate = Coordinate { row: 0, col: 1 };
    pub const TOP_RIGHT: Coordinate = Coordinate { row: 0, col: 2 };

    pub const MIDDLE_LEFT: Coordinate = Coordinate { row: 1, col: 0 };
    pub const MIDDLE_CENTER: Coordinate = Coordinate { row: 1, col: 1 };
    pub const MIDDLE_RIGHT: Coordinate = Coordinate { row: 1, col: 2 };

    pub const BOTTOM_LEFT: Coordinate = Coordinate { row: 2, col: 0 };
    pub const BOTTOM_CENTER: Coordinate = Coordinate { row: 2, col: 1 };
    pub const BOTTOM_RIGHT: Coordinate = Coordinate { row: 2, col: 2 };
}

impl GameBoard {
    pub fn new() -> Self {
        Self {
            cells: [[CellState::default(); 3]; 3],
        }
    }

    pub fn get_cell(&self, coordinate: Coordinate) -> &CellState {
        &self.cells[coordinate.row()][coordinate.col()]
    }

    pub fn place_piece(&mut self, piece: &Piece, coordinate: (usize, usize)) -> Result<(), String> {
        let cell = &self.cells[coordinate.0][coordinate.1];

        if matches!(&cell, CellState::Owned(_)) {
            return Err(String::from("Spot on board is alrady occupied"));
        }

        self.cells[coordinate.0][coordinate.1] = CellState::Owned(piece.to_owned());
        Ok(())
    }

    pub fn determine_winner(&self) -> Option<Piece> {
        for row in self.grab_rows() {
            if let Some(piece) = Self::determine_winner_of_line(&row) {
                return Some(piece.to_owned());
            }
        }

        for column in self.grab_columns() {
            if let Some(piece) = Self::determine_winner_of_line(&column) {
                return Some(piece.to_owned());
            }
        }

        for diagonal in self.grab_diagonals() {
            if let Some(piece) = Self::determine_winner_of_line(&diagonal) {
                return Some(piece.to_owned());
            }
        }

        None
    }

    pub fn display(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    CellState::Empty => print!("[ ] "),
                    CellState::Owned(Piece::X) => print!("[X] "),
                    CellState::Owned(Piece::O) => print!("[O] "),
                }
            }
            println!();
        }
    }

    fn determine_winner_of_line<'a>(line: &'a [&'a CellState]) -> Option<&'a Piece> {
        // Lifetimes required here to guarantee that the outgoing type (`Piece`) doesn't
        // outlive the incoming type that it is tied to `CellState`.

        let first_piece = match &line[0] {
            CellState::Owned(piece) => Some(piece),
            CellState::Empty => None,
        };

        let distinguished_piece = first_piece?; // returns None from the function if first_piece doesn't exist

        if line
            .iter()
            .filter(|&cell| matches!(cell, CellState::Owned(piece) if piece == distinguished_piece))
            .count()
            == line.len()
        {
            Some(distinguished_piece)
        } else {
            None
        }
    }

    fn grab_rows(&self) -> Vec<Vec<&CellState>> {
        let row0: Vec<_> = self.cells[0].iter().collect();
        let row1: Vec<_> = self.cells[1].iter().collect();
        let row2: Vec<_> = self.cells[2].iter().collect();

        vec![row0, row1, row2]
    }

    fn grab_columns(&self) -> Vec<Vec<&CellState>> {
        let column1: Vec<_> = self.cells.iter().map(|row| &row[1..=1][0]).collect();
        let column0: Vec<_> = self.cells.iter().map(|row| &row[0..=0][0]).collect();
        let column2: Vec<_> = self.cells.iter().map(|row| &row[2..=2][0]).collect();

        vec![column0, column1, column2]
    }

    fn grab_diagonals(&self) -> Vec<Vec<&CellState>> {
        let left_to_right_diagonal: Vec<_> = vec![
            self.get_cell(GameBoard::TOP_LEFT),
            self.get_cell(GameBoard::MIDDLE_CENTER),
            self.get_cell(GameBoard::BOTTOM_RIGHT),
        ];

        let right_to_left_diagonal: Vec<_> = vec![
            self.get_cell(GameBoard::TOP_RIGHT),
            self.get_cell(GameBoard::MIDDLE_CENTER),
            self.get_cell(GameBoard::BOTTOM_LEFT),
        ];

        vec![left_to_right_diagonal, right_to_left_diagonal]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_board_is_empty() {
        let game_board = GameBoard::new();

        for row in &game_board.cells {
            for cell in row {
                assert_eq!(*cell, CellState::Empty);
            }
        }
    }

    #[test]
    fn test_place_x_on_empty_cell() {
        let mut game_board = GameBoard::new();
        let result = game_board.place_piece(&Piece::X, (0, 0));

        assert_eq!(result, Ok(()));
        assert_eq!(game_board.cells[0][0], CellState::Owned(Piece::X));
    }

    #[test]
    fn test_place_o_on_empty_cell() {
        let mut game_board = GameBoard::new();
        let result = game_board.place_piece(&Piece::O, (0, 0));

        assert_eq!(result, Ok(()));
        assert_eq!(game_board.cells[0][0], CellState::Owned(Piece::O));
    }

    #[test]
    fn test_place_piece_on_owned_cell() {
        let mut game_board = GameBoard::new();
        assert!(game_board.place_piece(&Piece::O, (0, 0)).is_ok());

        let result = game_board.place_piece(&Piece::X, (0, 0));
        assert!(result.is_err());

        assert_eq!(game_board.cells[0][0], CellState::Owned(Piece::O));
    }

    #[test]
    fn test_empty_board_has_no_winner() {
        let game_board = GameBoard::new();

        assert_eq!(None, game_board.determine_winner());
    }

    #[test]
    fn test_board_with_a_winning_row_has_a_winner() {
        let mut game_board = GameBoard::new();
        let x = &Piece::X;

        let result = game_board
            .place_piece(x, (0, 0))
            .and_then(|_| game_board.place_piece(x, (0, 1)))
            .and_then(|_| game_board.place_piece(x, (0, 2)));

        assert!(result.is_ok());
        assert_eq!(Some(x.to_owned()), game_board.determine_winner());
    }

    #[test]
    fn test_board_with_a_winning_column_has_a_winner() {
        let mut game_board = GameBoard::new();
        let x = &Piece::X;

        let result = game_board
            .place_piece(x, (0, 0))
            .and_then(|_| game_board.place_piece(x, (1, 0)))
            .and_then(|_| game_board.place_piece(x, (2, 0)));

        assert!(result.is_ok());
        assert_eq!(Some(x.to_owned()), game_board.determine_winner());
    }

    #[test]
    fn test_board_with_a_winning_forward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::new();
        let x = &Piece::X;

        for index in 0..=2 {
            _ = game_board.place_piece(x, (index, index));
        }

        assert_eq!(Some(x.to_owned()), game_board.determine_winner());
    }

    #[test]
    fn test_board_with_a_winning_backward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::new();
        let x = &Piece::X;

        let backward_diagonal_indicies = (0..=2).zip((0..=2).rev()).enumerate();

        for (_, (row_index, column_index)) in backward_diagonal_indicies {
            _ = game_board.place_piece(x, (row_index, column_index));
        }

        assert_eq!(Some(x.to_owned()), game_board.determine_winner());
    }
}
