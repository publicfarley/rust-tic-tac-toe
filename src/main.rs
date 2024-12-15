mod game_model;

use crate::game_model::{place_piece, winner, GameBoard, Piece};

fn main() {
    let mut game_board: GameBoard = Default::default();
    println!("Hello game board: {game_board:#?}");

    _ = place_piece(&Piece::O, (0, 0), &mut game_board);
    println!("game_board[0]: {:#?}\n", game_board[0]);
    println!("Row 0: ");

    let mut game_board2: GameBoard = Default::default();

    let mut result = Err(String::new());
    for column in 0..=2 {
        result = place_piece(&Piece::X, (0, column), &mut game_board2);
    }

    println!("game_board[0]: {:#?}\n", game_board2[0]);

    if let Err(error) = result {
        println!(
            "Could not place X on row 0 of board: {:#?}. Got error: {}",
            &game_board2, error
        );
    } else {
        println!("Winner: {:#?}", winner(&game_board2));
    }
}
