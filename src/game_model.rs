use rand::Rng;
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    X,
    O,
}

impl Piece {
    pub const fn name(self) -> &'static str {
        match self {
            Self::O => "O",
            Self::X => "X",
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        output.push_str(self.name());

        write!(f, "{output}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CellState {
    Empty,
    Occupied(Piece),
}

impl Default for CellState {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Player {
    Computer(Piece),
    Human(Piece),
}

impl Player {
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Computer(_) => "Computer",
            Self::Human(_) => "Human",
        }
    }

    pub const fn piece(&self) -> &Piece {
        match self {
            Self::Human(piece) | Self::Computer(piece) => piece,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Coin {
    Heads,
    Tails,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PlayerID {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GameBoard {
    player_1: Player,
    player_2: Player,
    pub next_up: PlayerID,
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
    pub const fn row(&self) -> usize {
        self.row
    }

    pub const fn col(&self) -> usize {
        self.col
    }
}

#[allow(dead_code)]
impl Coordinate {
    pub const TOP_LEFT: Self = Self { row: 0, col: 0 };
    pub const TOP_CENTER: Self = Self { row: 0, col: 1 };
    pub const TOP_RIGHT: Self = Self { row: 0, col: 2 };

    pub const MIDDLE_LEFT: Self = Self { row: 1, col: 0 };
    pub const MIDDLE_CENTER: Self = Self { row: 1, col: 1 };
    pub const MIDDLE_RIGHT: Self = Self { row: 1, col: 2 };

    pub const BOTTOM_LEFT: Self = Self { row: 2, col: 0 };
    pub const BOTTOM_CENTER: Self = Self { row: 2, col: 1 };
    pub const BOTTOM_RIGHT: Self = Self { row: 2, col: 2 };
}

pub enum GameState<'a> {
    Winner(&'a Player),
    Draw,
    InProgress,
}

impl GameBoard {
    pub const POSITIONS: RangeInclusive<usize> = 1..=9;

    pub fn new() -> Self {
        let random_piece = Self::random_piece();
        let human = Player::Human(random_piece);
        let computer = Player::Computer(Self::other_piece(random_piece));

        Self {
            player_1: human,
            player_2: computer,
            next_up: Self::random_player_from(PlayerID::Player1, PlayerID::Player2),
            cells: [[CellState::default(); 3]; 3],
        }
    }

    pub fn player_for_id(&self, player_id: PlayerID) -> &Player {
        if player_id == PlayerID::Player1 {
            &self.player_1
        } else {
            &self.player_2
        }
    }

    pub fn get_cell_at_position(&self, position: usize) -> Option<&CellState> {
        let coordinate = Self::coordinate_at_position(position)?;

        Some(&self.cells[coordinate.row()][coordinate.col()])
    }

    fn set_cell_at_position(&mut self, cell: CellState, position: usize) -> Result<(), String> {
        let Some(coordinate) = Self::coordinate_at_position(position) else {
            return Err(format!("{position} is not a valid game board position"));
        };

        self.cells[coordinate.row()][coordinate.col()] = cell;

        Ok(())
    }

    pub fn play_next_up_at_position(&mut self, position: usize) -> Result<(), String> {
        let Some(cell) = self.get_cell_at_position(position) else {
            return Err(format!("{position} is not a valid game board position"));
        };

        if matches!(&cell, CellState::Occupied(_)) {
            return Err(String::from("Spot on board is already occupied"));
        }

        let next_piece = match self.next_up {
            PlayerID::Player1 => match self.player_1 {
                Player::Computer(piece) | Player::Human(piece) => piece,
            },
            PlayerID::Player2 => match self.player_2 {
                Player::Computer(piece) | Player::Human(piece) => piece,
            },
        };

        self.set_cell_at_position(CellState::Occupied(next_piece), position)?;

        self.next_up = match self.next_up {
            PlayerID::Player1 => PlayerID::Player2,
            PlayerID::Player2 => PlayerID::Player1,
        };

        Ok(())
    }

    pub fn is_computers_turn(&self) -> bool {
        let next_player = self.player_for_id(self.next_up);

        match next_player {
            Player::Computer(_) => true,
            Player::Human(_) => false,
        }
    }

    fn determine_winner(&self) -> Option<&Piece> {
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

    pub fn determine_winning_player(&self) -> Option<&Player> {
        let winning_piece = self.determine_winner()?;

        if self.player_1.piece() == winning_piece {
            Some(&self.player_1)
        } else {
            Some(&self.player_2)
        }
    }

    const fn coordinate_at_position(position: usize) -> Option<Coordinate> {
        match position {
            1 => Some(Coordinate::TOP_LEFT),
            3 => Some(Coordinate::TOP_RIGHT),
            4 => Some(Coordinate::MIDDLE_LEFT),
            2 => Some(Coordinate::TOP_CENTER),
            5 => Some(Coordinate::MIDDLE_CENTER),
            6 => Some(Coordinate::MIDDLE_RIGHT),
            7 => Some(Coordinate::BOTTOM_LEFT),
            8 => Some(Coordinate::BOTTOM_CENTER),
            9 => Some(Coordinate::BOTTOM_RIGHT),
            _ => None,
        }
    }

    pub fn get_available_positions(&self) -> Vec<usize> {
        let mut available_positions: Vec<usize> = Vec::new();

        Self::POSITIONS.for_each(|position| {
            if let Some(cell) = self.get_cell_at_position(position) {
                if *cell == CellState::Empty {
                    available_positions.push(position);
                }
            }
        });

        available_positions
    }

    pub fn is_board_full(&self) -> bool {
        self.get_available_positions().is_empty()
    }

    pub fn get_random_available_position(&self) -> Option<usize> {
        let available_positions = self.get_available_positions();

        if available_positions.is_empty() {
            return None;
        };

        available_positions
            .get(rand::thread_rng().gen_range(0..available_positions.len()))
            .copied()
    }

    pub fn game_state(&self) -> GameState {
        self.determine_winning_player().map_or_else(
            || {
                if self.is_board_full() {
                    GameState::Draw
                } else {
                    GameState::InProgress
                }
            },
            GameState::Winner,
        )
    }

    pub fn is_game_over(&self) -> bool {
        self.determine_winning_player().is_some() || self.is_board_full()
    }

    fn determine_winner_of_line<'a>(line: &[&'a CellState]) -> Option<&'a Piece> {
        // Lifetimes required here to guarantee that the outgoing type (`Piece`) doesn't
        // outlive the incoming type that it is tied to `CellState`.

        let first_piece = match &line[0] {
            CellState::Occupied(piece) => Some(piece),
            CellState::Empty => None,
        };

        let distinguished_piece = first_piece?; // returns None from the function if first_piece doesn't exist

        if line
            .iter()
            .filter(
                |&cell| matches!(cell, CellState::Occupied(piece) if piece == distinguished_piece),
            )
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

    fn random_player_from(player_1: PlayerID, player_2: PlayerID) -> PlayerID {
        if Self::flip_coin() == Coin::Heads {
            player_1
        } else {
            player_2
        }
    }

    fn random_piece() -> Piece {
        if Self::flip_coin() == Coin::Heads {
            Piece::O
        } else {
            Piece::X
        }
    }

    fn other_piece(piece: Piece) -> Piece {
        if piece == Piece::O {
            Piece::X
        } else {
            Piece::O
        }
    }

    fn flip_coin() -> Coin {
        let mut random_number_generator = rand::thread_rng();
        let zero_or_one: u8 = random_number_generator.gen_range(0..=1);

        if zero_or_one == 0 {
            Coin::Heads
        } else {
            Coin::Tails
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
                    CellState::Occupied(piece) => {
                        let occupied_cell = format!("[{}] ", piece.name());
                        output.push_str(occupied_cell.as_str());
                    }
                }
            }
            output.push('\n');
        }

        output.pop();
        write!(f, "{output}")
    }
}

pub fn execute_computer_turn(game_board: &mut GameBoard) -> Result<(), String> {
    game_board.get_random_available_position().map_or_else(
        || Err("No available positions".to_string()),
        |position| {
            // display_spinner_with_message("The computer is thinking...");
            let piece = game_board.player_for_id(game_board.next_up).piece();
            println!("\nThe computer played {piece} in position: {position}");
            game_board.play_next_up_at_position(position)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use Piece::O;

    #[test]
    fn test_initial_board_is_empty() {
        let game_board = GameBoard::new();

        let test_cell_at = |position: usize| {
            let cell = game_board.get_cell_at_position(position);
            assert_eq!(cell, Some(&CellState::Empty));
        };

        GameBoard::POSITIONS.for_each(test_cell_at);
    }

    #[test]
    fn test_place_x_on_empty_cell() {
        let mut game_board = new_with_first_up(Player::Computer(Piece::X));
        let result = game_board.play_next_up_at_position(1);

        assert!(result.is_ok());
        assert_eq!(
            game_board.get_cell_at_position(1),
            Some(&CellState::Occupied(Piece::X))
        );
    }

    #[test]
    fn test_place_o_on_empty_cell() {
        let mut game_board = new_with_first_up(Player::Computer(O));
        let result = game_board.play_next_up_at_position(1);

        assert!(result.is_ok());
        assert_eq!(
            game_board.get_cell_at_position(1),
            Some(&CellState::Occupied(O))
        );
    }

    #[test]
    fn test_place_piece_on_occupied_cell() {
        let mut game_board = new_with_first_up(Player::Computer(O));

        assert!(game_board.play_next_up_at_position(1).is_ok());
        assert!(game_board.play_next_up_at_position(1).is_err());

        assert_eq!(
            game_board.get_cell_at_position(1),
            Some(&CellState::Occupied(O))
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

        let result = game_board
            .play_next_up_at_position(1)
            .and_then(|_| game_board.play_next_up_at_position(4))
            .and_then(|_| game_board.play_next_up_at_position(2))
            .and_then(|_| game_board.play_next_up_at_position(7))
            .and_then(|_| game_board.play_next_up_at_position(3));

        assert!(result.is_ok());

        // row: 1,2,3
        assert!(game_board.determine_winner().is_some());
    }

    #[test]
    fn test_board_with_a_winning_column_has_a_winner() {
        let mut game_board = GameBoard::new();

        let result = game_board
            .play_next_up_at_position(1)
            .and_then(|_| game_board.play_next_up_at_position(2))
            .and_then(|_| game_board.play_next_up_at_position(4))
            .and_then(|_| game_board.play_next_up_at_position(9))
            .and_then(|_| game_board.play_next_up_at_position(7));

        assert!(result.is_ok());

        // column: 1,4,7
        assert!(game_board.determine_winner().is_some());
    }

    #[test]
    fn test_board_with_a_winning_forward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::new();

        let result = game_board
            .play_next_up_at_position(1)
            .and_then(|_| game_board.play_next_up_at_position(2))
            .and_then(|_| game_board.play_next_up_at_position(5))
            .and_then(|_| game_board.play_next_up_at_position(7))
            .and_then(|_| game_board.play_next_up_at_position(9));

        assert!(result.is_ok());

        // diagonal: 1,5,9
        assert!(game_board.determine_winner().is_some());
    }

    #[test]
    fn test_board_with_a_winning_backward_diagonal_has_a_winner() {
        let mut game_board = GameBoard::new();

        let result = game_board
            .play_next_up_at_position(3)
            .and_then(|_| game_board.play_next_up_at_position(2))
            .and_then(|_| game_board.play_next_up_at_position(5))
            .and_then(|_| game_board.play_next_up_at_position(1))
            .and_then(|_| game_board.play_next_up_at_position(7));

        assert!(result.is_ok());

        // diagonal: 3,5,7
        assert!(game_board.determine_winner().is_some());
    }

    #[test]
    fn test_invalid_cell_positions_return_none() {
        let game_board = GameBoard::new();

        assert_eq!(game_board.get_cell_at_position(0), None);

        assert_eq!(game_board.get_cell_at_position(100), None);
    }

    #[test]
    fn test_valid_cell_positions_return_valid_cells() {
        let game_board = GameBoard::new();

        assert_eq!(
            game_board.get_cell_at_position(1),
            Some(get_cell_at_coordinate(&game_board, &Coordinate::TOP_LEFT))
        );
        assert_eq!(
            game_board.get_cell_at_position(2),
            Some(get_cell_at_coordinate(&game_board, &Coordinate::TOP_CENTER))
        );
        assert_eq!(
            game_board.get_cell_at_position(3),
            Some(get_cell_at_coordinate(&game_board, &Coordinate::TOP_RIGHT))
        );

        assert_eq!(
            game_board.get_cell_at_position(4),
            Some(get_cell_at_coordinate(
                &game_board,
                &Coordinate::MIDDLE_LEFT
            ))
        );
        assert_eq!(
            game_board.get_cell_at_position(5),
            Some(get_cell_at_coordinate(
                &game_board,
                &Coordinate::MIDDLE_CENTER
            ))
        );
        assert_eq!(
            game_board.get_cell_at_position(6),
            Some(get_cell_at_coordinate(
                &game_board,
                &Coordinate::MIDDLE_RIGHT
            ))
        );

        assert_eq!(
            game_board.get_cell_at_position(7),
            Some(get_cell_at_coordinate(
                &game_board,
                &Coordinate::BOTTOM_LEFT
            ))
        );
        assert_eq!(
            game_board.get_cell_at_position(8),
            Some(get_cell_at_coordinate(
                &game_board,
                &Coordinate::BOTTOM_CENTER
            ))
        );
        assert_eq!(
            game_board.get_cell_at_position(9),
            Some(get_cell_at_coordinate(
                &game_board,
                &Coordinate::BOTTOM_RIGHT
            ))
        );
    }

    #[test]
    fn test_player_id() {
        let game_board = GameBoard::new();

        assert_eq!(
            *game_board.player_for_id(PlayerID::Player1),
            game_board.player_1
        );
        assert_eq!(
            *game_board.player_for_id(PlayerID::Player2),
            game_board.player_2
        );
    }

    #[test]
    fn test_determine_winning_player_resolves_to_x_if_x_wins() {
        let mut game_board = new_with_first_up(Player::Computer(Piece::X));
        let result = first_player_top_row_win(&mut game_board);

        assert!(result.is_ok());

        assert!(matches!(
            game_board.determine_winning_player(),
            Some(&Player::Computer(Piece::X))
        ));
    }

    #[test]
    fn test_determine_winning_player_resolves_to_o_if_o_wins() {
        let mut game_board = new_with_first_up(Player::Computer(O));
        let result = first_player_top_row_win(&mut game_board);

        assert!(result.is_ok());

        assert!(matches!(
            game_board.determine_winning_player(),
            Some(&Player::Computer(Piece::O))
        ));
    }

    #[test]
    fn test_determine_winning_player_resolves_to_none_if_there_is_no_winner() {
        let mut game_board = new_with_first_up(Player::Computer(O));

        let result = game_board.play_next_up_at_position(1);

        assert!(result.is_ok());
        assert!(matches!(game_board.determine_winning_player(), None));
    }

    #[test]
    fn test_winning_player_can_be_player_2() {
        let first_up = Player::Computer(O);
        let _second_up = Player::Human(Piece::X);

        let mut game_board = new_with_first_up(first_up);

        let result = first_player_top_row_win(&mut game_board);

        assert!(result.is_ok());
        assert!(matches!(
            game_board.determine_winning_player(),
            Some(_second_up)
        ));
    }

    #[test]
    fn test_board_is_full() {
        let mut game_board = GameBoard::new();
        GameBoard::POSITIONS.for_each(|position| _ = game_board.play_next_up_at_position(position));

        assert!(game_board.is_board_full());
    }

    #[test]
    fn test_empty_board_is_not_full() {
        let game_board = GameBoard::new();

        assert!(!game_board.is_board_full());
    }

    #[test]
    fn test_board_with_less_than_9_positions_is_not_full() {
        let mut game_board = GameBoard::new();

        let all_positions = GameBoard::POSITIONS;
        let end_position = all_positions.end();
        let new_end_position = *end_position - 1;
        let less_than_all_positions = *all_positions.start()..=new_end_position;

        less_than_all_positions
            .for_each(|position| _ = game_board.play_next_up_at_position(position));

        assert!(!game_board.is_board_full());
    }

    #[test]
    fn test_get_random_available_position_is_a_not_none() {
        let random_position = GameBoard::new().get_random_available_position();

        assert!(matches!(random_position, Some(_)));
    }

    #[test]
    fn test_name_of_human_player() {
        let human_name = "Human";

        assert_eq!(human_name, Player::Human(Piece::X).name());
        assert_eq!(human_name, Player::Human(O).name());
    }

    #[test]
    fn test_name_of_computer_player() {
        let computer_name = "Computer";

        assert_eq!(computer_name, Player::Computer(Piece::X).name());
        assert_eq!(computer_name, Player::Computer(O).name());
    }

    #[test]
    fn test_piece_retrieval() {
        assert_eq!(&Piece::X, Player::Computer(Piece::X).piece());
        assert_eq!(&O, Player::Computer(O).piece());
        assert_eq!(&Piece::X, Player::Human(Piece::X).piece());
        assert_eq!(&O, Player::Human(O).piece());
    }

    // Private test utility functions
    fn first_player_top_row_win(game_board: &mut GameBoard) -> Result<(), String> {
        game_board
            .play_next_up_at_position(1)
            .and_then(|_| game_board.play_next_up_at_position(4))
            .and_then(|_| game_board.play_next_up_at_position(2))
            .and_then(|_| game_board.play_next_up_at_position(7))
            .and_then(|_| game_board.play_next_up_at_position(3))
    }

    const fn get_cell_at_coordinate<'a>(
        game_board: &'a GameBoard,
        coordinate: &Coordinate,
    ) -> &'a CellState {
        &game_board.cells[coordinate.row()][coordinate.col()]
    }

    fn new_with_first_up(first_up_player: Player) -> GameBoard {
        let other_player = match first_up_player {
            Player::Computer(piece) => Player::Human(GameBoard::other_piece(piece)),
            Player::Human(piece) => Player::Computer(GameBoard::other_piece(piece)),
        };

        GameBoard {
            player_1: first_up_player,
            player_2: other_player,
            next_up: PlayerID::Player1,
            cells: [[CellState::default(); 3]; 3],
        }
    }
}
