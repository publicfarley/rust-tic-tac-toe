#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Piece {
    X,
    O,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CellState {
    Empty,
    Owned(Piece),
}

impl Default for CellState {
    fn default() -> Self {
        Self::Empty
    }
}

pub type GameBoard = [[CellState; 3]; 3];

pub fn place_piece(
    piece: &Piece,
    coordinate: (usize, usize),
    board: &mut GameBoard,
) -> Result<(), String> {
    if matches!(&board[coordinate.0][coordinate.1], CellState::Owned(_)) {
        return Err(String::from("Spot on board is alrady occupied"));
    }

    board[coordinate.0][coordinate.1] = CellState::Owned(piece.to_owned());
    Ok(())
}

pub fn determine_winner_of_line<'a>(line: &'a [&'a CellState]) -> Option<&'a Piece> {
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

pub fn determine_winner(board: &GameBoard) -> Option<Piece> {
    for row in grab_rows(board) {
        if let Some(piece) = determine_winner_of_line(&row) {
            return Some(piece.to_owned());
        }
    }

    for column in grab_columns(board) {
        if let Some(piece) = determine_winner_of_line(&column) {
            return Some(piece.to_owned());
        }
    }

    for diagonal in grab_diagonals(board) {
        if let Some(piece) = determine_winner_of_line(&diagonal) {
            return Some(piece.to_owned());
        }
    }

    None
}

fn grab_rows(board: &GameBoard) -> Vec<Vec<&CellState>> {
    let row0: Vec<_> = board[0].iter().collect();
    let row1: Vec<_> = board[1].iter().collect();
    let row2: Vec<_> = board[2].iter().collect();

    vec![row0, row1, row2]
}

fn grab_columns(board: &GameBoard) -> Vec<Vec<&CellState>> {
    let column0: Vec<_> = board.iter().map(|row| &row[0..=0][0]).collect();
    let column1: Vec<_> = board.iter().map(|row| &row[1..=1][0]).collect();
    let column2: Vec<_> = board.iter().map(|row| &row[2..=2][0]).collect();

    vec![column0, column1, column2]
}

fn grab_diagonals(board: &GameBoard) -> Vec<Vec<&CellState>> {
    let left_to_right_diagonal: Vec<_> = vec![&board[0][0], &board[1][1], &board[2][2]];
    let right_to_left_diagonal: Vec<_> = vec![&board[0][2], &board[1][1], &board[2][0]];

    vec![left_to_right_diagonal, right_to_left_diagonal]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_board_is_empty() {
        let game_board = GameBoard::default();

        for row in &game_board {
            for cell in row {
                assert_eq!(*cell, CellState::Empty);
            }
        }
    }

    #[test]
    fn test_place_x_on_empty_cell() {
        let mut game_board = GameBoard::default();
        game_board[0][0] = CellState::Owned(Piece::X);

        let mut game_board2 = GameBoard::default();
        let result = place_piece(&Piece::X, (0, 0), &mut game_board2);

        assert_eq!(result, Ok(()));
        assert_eq!(game_board[0][0], game_board2[0][0]);
    }

    #[test]
    fn test_place_o_on_empty_cell() {
        let mut game_board = GameBoard::default();
        game_board[0][0] = CellState::Owned(Piece::O);

        let mut game_board2 = GameBoard::default();
        let result = place_piece(&Piece::O, (0, 0), &mut game_board2);

        assert_eq!(result, Ok(()));
        assert_eq!(game_board[0][0], game_board2[0][0]);
    }

    #[test]
    fn test_place_piece_on_owned_cell() {
        let mut game_board = GameBoard::default();
        game_board[0][0] = CellState::Owned(Piece::O);
        let board_with_only_o_placed = game_board.clone();

        let result = place_piece(&Piece::X, (0, 0), &mut game_board);

        assert!(result.is_err());
        assert_eq!(game_board, board_with_only_o_placed);
    }

    #[test]
    fn test_empty_row_has_no_winner() {
        let game_board = GameBoard::default();
        let rows = grab_rows(&game_board);

        assert_eq!(None, determine_winner_of_line(&rows[0]));
    }

    #[test]
    fn test_full_row_of_x_has_winner_x() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        let result = place_piece(x, (0, 0), &mut game_board)
            .and_then(|_| place_piece(x, (0, 1), &mut game_board))
            .and_then(|_| place_piece(x, (0, 2), &mut game_board));

        assert_eq!(result, Ok(()));

        let rows = grab_rows(&game_board);
        let first_row = &rows[0];

        assert_eq!(Some(x), determine_winner_of_line(&first_row));
    }

    #[test]
    fn test_full_row_of_o_has_winner_o() {
        let mut game_board = GameBoard::default();
        let o = &Piece::O;

        let result = place_piece(o, (0, 0), &mut game_board)
            .and_then(|_| place_piece(o, (0, 1), &mut game_board))
            .and_then(|_| place_piece(o, (0, 2), &mut game_board));

        assert_eq!(result, Ok(()));

        let rows = grab_rows(&game_board);
        let first_row = &rows[0];

        assert_eq!(Some(o), determine_winner_of_line(&first_row));
    }

    #[test]
    fn test_empty_board_has_no_winner() {
        let game_board = GameBoard::default();

        assert_eq!(None, determine_winner(&game_board));
    }

    #[test]
    fn test_board_with_a_winning_row_has_a_winner() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        let result = place_piece(x, (0, 0), &mut game_board)
            .and_then(|_| place_piece(x, (0, 1), &mut game_board))
            .and_then(|_| place_piece(x, (0, 2), &mut game_board));

        assert!(result.is_ok());
        assert_eq!(Some(x.to_owned()), determine_winner(&game_board));
    }

    #[test]
    fn test_board_with_a_winning_column_has_a_winner() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        let result = place_piece(x, (0, 0), &mut game_board)
            .and_then(|_| place_piece(x, (1, 0), &mut game_board))
            .and_then(|_| place_piece(x, (2, 0), &mut game_board));

        assert!(result.is_ok());
        assert_eq!(Some(x.to_owned()), determine_winner(&game_board));
    }

    #[test]
    fn test_board_with_a_winning_forward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        for index in 0..=2 {
            _ = place_piece(x, (index, index), &mut game_board);
        }

        assert_eq!(Some(x.to_owned()), determine_winner(&game_board));
    }

    #[test]
    fn test_board_with_a_winning_backward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        let backward_diagonal_indicies = (0..=2).zip((0..=2).rev()).enumerate();

        for (_, (row_index, column_index)) in backward_diagonal_indicies {
            _ = place_piece(x, (row_index, column_index), &mut game_board);
        }

        assert_eq!(Some(x.to_owned()), determine_winner(&game_board));
    }
}
