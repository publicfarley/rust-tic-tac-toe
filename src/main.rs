mod game_model;

use crate::game_model::{determine_winner_of_line, place_piece, GameBoard, Piece};

fn main() {
    let mut game_board: GameBoard = Default::default();
    println!("Hello game board: {game_board:#?}");

    place_piece(&Piece::O, (0, 0), &mut game_board);
    println!("game_board[0]: {:#?}\n", game_board[0]);

    println!("Row 0: ");
    println!("Winner: {:#?}", determine_winner_of_line(&game_board[0]));

    let mut game_board2: GameBoard = Default::default();

    for column in 0..=2 {
        place_piece(&Piece::X, (0, column), &mut game_board2);
    }

    println!("game_board[0]: {:#?}\n", game_board2[0]);

    println!("Row 0: ");
    println!("Winner: {:#?}", determine_winner_of_line(&game_board2[0]));
}
