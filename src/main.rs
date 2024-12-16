mod game_model;

use crate::game_model::{GameBoard, Piece};

fn main() {
    let mut game_board = GameBoard::new();
    println!("Hello game board: {game_board:#?}");

    _ = game_board.place_piece(&Piece::O, (0, 0));
    println!("game_board[0]: {:#?}\n", game_board.cells[0]);
    println!("Row 0: ");

    let mut game_board2 = GameBoard::new();

    let mut result = Err(String::new());
    for column in 0..=2 {
        result = game_board2.place_piece(&Piece::X, (0, column));
    }

    println!("game_board[0]: {:#?}\n", game_board2.cells[0]);

    if let Err(error) = result {
        println!(
            "Could not place X on row 0 of board: {:#?}. Got error: {}",
            &game_board2, error
        );
    } else {
        println!("Winner: {:#?}", game_board2.determine_winner());
    }

    game_board2.display();
}
