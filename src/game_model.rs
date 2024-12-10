#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CellState {
    Empty,
    X,
    O,
}

impl CellState {
    pub const fn all_cases() -> &'static [Self] {
        &[Self::Empty, Self::X, Self::O]
    }
}

impl Default for CellState {
    fn default() -> Self {
        Self::Empty
    }
}

pub type GameBoard = [[CellState; 3]; 3];

fn place_piece(coordinate: (usize, usize), piece: &CellState, board: &GameBoard) -> GameBoard {
    let mut new_game_board = board.clone();
    new_game_board[coordinate.0][coordinate.1] = piece.to_owned();

    return new_game_board;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_x() {
        let mut game_board = GameBoard::default();
        game_board[0][0] = CellState::X;

        let game_board2 = GameBoard::default();
        let result_of_placing_x = place_piece((0, 0), &CellState::X, &game_board2);
        assert_eq!(game_board[0][0], result_of_placing_x[0][0]);
    }
}
