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

impl GameBoard {
    pub fn new() -> Self {
        Self {
            cells: [[CellState::default(); 3]; 3],
        }
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Option<&CellState> {
        self.cells.get(row).and_then(|r| r.get(col))
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
        for row in grab_rows(self) {
            if let Some(piece) = determine_winner_of_line(&row) {
                return Some(piece.to_owned());
            }
        }

        for column in grab_columns(self) {
            if let Some(piece) = determine_winner_of_line(&column) {
                return Some(piece.to_owned());
            }
        }

        for diagonal in grab_diagonals(self) {
            if let Some(piece) = determine_winner_of_line(&diagonal) {
                return Some(piece.to_owned());
            }
        }

        None
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

fn grab_rows(board: &GameBoard) -> Vec<Vec<&CellState>> {
    let row0: Vec<_> = board.cells[0].iter().collect();
    let row1: Vec<_> = board.cells[1].iter().collect();
    let row2: Vec<_> = board.cells[2].iter().collect();

    vec![row0, row1, row2]
}

fn grab_columns(board: &GameBoard) -> Vec<Vec<&CellState>> {
    let column0: Vec<_> = board.cells.iter().map(|row| &row[0..=0][0]).collect();
    let column1: Vec<_> = board.cells.iter().map(|row| &row[1..=1][0]).collect();
    let column2: Vec<_> = board.cells.iter().map(|row| &row[2..=2][0]).collect();

    vec![column0, column1, column2]
}

fn grab_diagonals(board: &GameBoard) -> Vec<Vec<&CellState>> {
    let left_to_right_diagonal: Vec<_> =
        vec![&board.cells[0][0], &board.cells[1][1], &board.cells[2][2]];
    let right_to_left_diagonal: Vec<_> =
        vec![&board.cells[0][2], &board.cells[1][1], &board.cells[2][0]];

    vec![left_to_right_diagonal, right_to_left_diagonal]
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
    fn test_empty_row_has_no_winner() {
        let game_board = GameBoard::new();
        let rows = grab_rows(&game_board);

        assert_eq!(None, determine_winner_of_line(&rows[0]));
    }

    #[test]
    fn test_full_row_of_x_has_winner_x() {
        let mut game_board = GameBoard::new();
        let x = &Piece::X;

        let result = game_board
            .place_piece(x, (0, 0))
            .and_then(|_| game_board.place_piece(x, (0, 1)))
            .and_then(|_| game_board.place_piece(x, (0, 2)));

        assert_eq!(result, Ok(()));

        let rows = grab_rows(&game_board);
        let first_row = &rows[0];

        assert_eq!(Some(x), determine_winner_of_line(&first_row));
    }

    #[test]
    fn test_full_row_of_o_has_winner_o() {
        let mut game_board = GameBoard::new();
        let o = &Piece::O;

        let result = game_board
            .place_piece(o, (0, 0))
            .and_then(|_| game_board.place_piece(o, (0, 1)))
            .and_then(|_| game_board.place_piece(o, (0, 2)));

        assert_eq!(result, Ok(()));

        let rows = grab_rows(&game_board);
        let first_row = &rows[0];

        assert_eq!(Some(o), determine_winner_of_line(&first_row));
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
