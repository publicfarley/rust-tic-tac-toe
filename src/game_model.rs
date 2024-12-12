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

pub fn determine_winner_of_line(line: &[CellState; 3]) -> Option<&Piece> {
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

        assert_eq!(None, determine_winner_of_line(&game_board[0]));
    }

    #[test]
    fn test_full_row_of_x_has_winner_x() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        let result = place_piece(x, (0, 0), &mut game_board)
            .and_then(|_| place_piece(x, (0, 1), &mut game_board))
            .and_then(|_| place_piece(x, (0, 2), &mut game_board));

        assert_eq!(result, Ok(()));
        assert_eq!(Some(x), determine_winner_of_line(&game_board[0]));
    }

    #[test]
    fn test_full_row_of_o_has_winner_o() {
        let mut game_board = GameBoard::default();
        let o = &Piece::O;

        let result = place_piece(o, (0, 0), &mut game_board)
            .and_then(|_| place_piece(o, (0, 1), &mut game_board))
            .and_then(|_| place_piece(o, (0, 2), &mut game_board));

        assert_eq!(result, Ok(()));
        assert_eq!(Some(o), determine_winner_of_line(&game_board[0]));
    }
}
