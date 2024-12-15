use std::ops::RangeInclusive;

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

const CELL_IDS: RangeInclusive<usize> = 0..=2;

pub fn is_winner(board: &GameBoard) -> bool {
    for row_index in CELL_IDS {
        let row = &board[row_index];

        if determine_winner_of_line(&row).is_some() {
            return true;
        }
    }

    for column_index in CELL_IDS {
        let column: &[CellState; 3] = &[
            board[0][column_index].clone(),
            board[1][column_index].clone(),
            board[2][column_index].clone(),
        ];

        if determine_winner_of_line(column).is_some() {
            return true;
        }
    }

    let column_0_diagonal: &[CellState; 3] = &[
        board[0][0].clone(),
        board[1][1].clone(),
        board[2][2].clone(),
    ];

    if determine_winner_of_line(column_0_diagonal).is_some() {
        return true;
    }

    let column_2_diagonal: &[CellState; 3] = &[
        board[0][2].clone(),
        board[1][1].clone(),
        board[2][0].clone(),
    ];

    if determine_winner_of_line(column_2_diagonal).is_some() {
        return true;
    }

    false
}

// fn grab_columns(board: &GameBoard) -> Vec<Vec<&CellState>> {
//     let mut columns_vector: Vec<Vec<&CellState>> = Vec::new();

//     for column_index in CELL_IDS {
//         let column = vec![
//             &board[0][column_index],
//             &board[1][column_index],
//             &board[2][column_index],
//         ];

//         columns_vector.push(column);
//     }

//     columns_vector
// }

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

    #[test]
    fn test_empty_board_has_no_winner() {
        let game_board = GameBoard::default();

        assert_eq!(false, is_winner(&game_board));
    }

    #[test]
    fn test_board_with_a_winning_row_has_a_winner() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        let result = place_piece(x, (0, 0), &mut game_board)
            .and_then(|_| place_piece(x, (0, 1), &mut game_board))
            .and_then(|_| place_piece(x, (0, 2), &mut game_board));

        assert!(result.is_ok());
        assert_eq!(true, is_winner(&game_board));
    }

    #[test]
    fn test_board_with_a_winning_column_has_a_winner() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        let result = place_piece(x, (0, 0), &mut game_board)
            .and_then(|_| place_piece(x, (1, 0), &mut game_board))
            .and_then(|_| place_piece(x, (2, 0), &mut game_board));

        assert!(result.is_ok());
        assert_eq!(true, is_winner(&game_board));
    }

    #[test]
    fn test_board_with_a_winning_forward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        for index in CELL_IDS {
            _ = place_piece(x, (index, index), &mut game_board);
        }

        assert_eq!(true, is_winner(&game_board));
    }

    #[test]
    fn test_board_with_a_winning_backward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::default();
        let x = &Piece::X;

        for (_, (row_index, column_index)) in CELL_IDS.zip(CELL_IDS.rev()).enumerate() {
            _ = place_piece(x, (row_index, column_index), &mut game_board);
        }

        assert_eq!(true, is_winner(&game_board));
    }
}
