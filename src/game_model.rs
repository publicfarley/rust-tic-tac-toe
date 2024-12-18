use rand::Rng;
use std::fmt;

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
    pub next_up: Piece,
    cells: [[CellState; 3]; 3],
}

pub struct Coordinate {
    row: usize,
    col: usize,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coordinate {{ row: {}, col: {} }}", self.row, self.col)
    }
}

impl Coordinate {
    pub const fn new(row: usize, col: usize) -> Option<Self> {
        if row < 3 && col < 3 {
            Some(Self { row, col })
        } else {
            None
        }
    }

    pub const fn row(&self) -> usize {
        self.row
    }

    pub const fn col(&self) -> usize {
        self.col
    }
}

#[allow(dead_code)]
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
            next_up: Self::random_piece(),
            cells: [[CellState::default(); 3]; 3],
        }
    }

    pub const fn get_cell(&self, coordinate: &Coordinate) -> &CellState {
        &self.cells[coordinate.row()][coordinate.col()]
    }

    pub fn place_piece(&mut self, piece: Piece, coordinate: &Coordinate) -> Result<(), String> {
        let cell = &self.cells[coordinate.row()][coordinate.col()];

        if matches!(&cell, CellState::Owned(_)) {
            return Err(String::from("Spot on board is alrady occupied"));
        }

        self.cells[coordinate.row()][coordinate.col()] = CellState::Owned(piece);
        Ok(())
    }

    pub fn determine_winner(&self) -> Option<&Piece> {
        for row in self.get_rows() {
            if let Some(piece) = Self::determine_winner_of_line(&row) {
                return Some(piece);
            }
        }

        for column in self.get_columns() {
            if let Some(piece) = Self::determine_winner_of_line(&column) {
                return Some(piece);
            }
        }

        for diagonal in self.get_diagonals() {
            if let Some(piece) = Self::determine_winner_of_line(&diagonal) {
                return Some(piece);
            }
        }

        None
    }

    fn determine_winner_of_line<'a>(line: &[&'a CellState]) -> Option<&'a Piece> {
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

    fn get_rows(&self) -> [[&CellState; 3]; 3] {
        std::array::from_fn(|row| std::array::from_fn(|col| &self.cells[row][col]))
    }

    fn get_columns(&self) -> [[&CellState; 3]; 3] {
        std::array::from_fn(|col| std::array::from_fn(|row| &self.cells[row][col]))
    }

    fn get_diagonals(&self) -> [[&CellState; 3]; 2] {
        let main_diagonal = std::array::from_fn(|i| &self.cells[i][i]);
        let anti_diagonal = std::array::from_fn(|i| &self.cells[i][2 - i]);

        [main_diagonal, anti_diagonal]
    }

    fn random_piece() -> Piece {
        let mut random_number_generator = rand::thread_rng();
        let zero_or_one: u8 = random_number_generator.gen_range(0..=1);

        if zero_or_one == 0 {
            Piece::O
        } else {
            Piece::X
        }
    }
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();

        for row in self.cells {
            for cell in row {
                match cell {
                    CellState::Empty => output.push_str("[ ] "),
                    CellState::Owned(Piece::X) => output.push_str("[X] "),
                    CellState::Owned(Piece::O) => output.push_str("[O] "),
                }
            }
            output.push('\n');
        }

        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_board_is_empty() {
        let game_board = GameBoard::new();

        let test_cell_at = |row: usize, col: usize| {
            let cell_coordinate = Coordinate::new(row, col)
                .expect("Expected Some(Coordinate), but got None, using index {row},{col}");
            let cell = game_board.get_cell(&cell_coordinate);
            assert_eq!(*cell, CellState::Empty);
        };

        for row in 0..=2 {
            for col in 0..=2 {
                test_cell_at(row, col);
            }
        }
    }

    #[test]
    fn test_place_x_on_empty_cell() {
        let mut game_board = GameBoard::new();
        let result = game_board.place_piece(Piece::X, &GameBoard::TOP_LEFT);

        assert_eq!(result, Ok(()));
        assert_eq!(
            *game_board.get_cell(&GameBoard::TOP_LEFT),
            CellState::Owned(Piece::X)
        );
    }

    #[test]
    fn test_place_o_on_empty_cell() {
        let mut game_board = GameBoard::new();
        let result = game_board.place_piece(Piece::O, &GameBoard::TOP_LEFT);

        assert_eq!(result, Ok(()));
        assert_eq!(
            *game_board.get_cell(&GameBoard::TOP_LEFT),
            CellState::Owned(Piece::O)
        );
    }

    #[test]
    fn test_place_piece_on_owned_cell() {
        let mut game_board = GameBoard::new();
        assert!(game_board
            .place_piece(Piece::O, &GameBoard::TOP_LEFT)
            .is_ok());

        let result = game_board.place_piece(Piece::X, &GameBoard::TOP_LEFT);
        assert!(result.is_err());

        assert_eq!(
            *game_board.get_cell(&GameBoard::TOP_LEFT),
            CellState::Owned(Piece::O)
        );
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
            .place_piece(*x, &GameBoard::TOP_LEFT)
            .and_then(|_| game_board.place_piece(*x, &GameBoard::TOP_CENTER))
            .and_then(|_| game_board.place_piece(*x, &GameBoard::TOP_RIGHT));

        assert!(result.is_ok());
        assert_eq!(Some(x), game_board.determine_winner());
    }

    #[test]
    fn test_board_with_a_winning_column_has_a_winner() {
        let mut game_board = GameBoard::new();
        let x = &Piece::X;

        let result = game_board
            .place_piece(*x, &GameBoard::TOP_LEFT)
            .and_then(|_| game_board.place_piece(*x, &GameBoard::MIDDLE_LEFT))
            .and_then(|_| game_board.place_piece(*x, &GameBoard::BOTTOM_LEFT));

        assert!(result.is_ok());
        assert_eq!(Some(x), game_board.determine_winner());
    }

    #[test]
    fn test_board_with_a_winning_forward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::new();
        let x = &Piece::X;

        _ = game_board
            .place_piece(*x, &GameBoard::TOP_LEFT)
            .and_then(|_| game_board.place_piece(*x, &GameBoard::MIDDLE_CENTER))
            .and_then(|_| game_board.place_piece(*x, &GameBoard::BOTTOM_RIGHT));

        assert_eq!(Some(x), game_board.determine_winner());
    }

    #[test]
    fn test_board_with_a_winning_backward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::new();
        let x = &Piece::X;

        _ = game_board
            .place_piece(*x, &GameBoard::TOP_RIGHT)
            .and_then(|_| game_board.place_piece(*x, &GameBoard::MIDDLE_CENTER))
            .and_then(|_| game_board.place_piece(*x, &GameBoard::BOTTOM_LEFT));

        assert_eq!(Some(x), game_board.determine_winner());
    }
}
