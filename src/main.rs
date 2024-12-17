mod game_model;

use crate::game_model::{GameBoard, Piece};

fn main() {
    let mut game_board = GameBoard::new();
    println!("Empty game board:");
    game_board.display();

    _ = game_board.place_piece(&Piece::O, (0, 0));
    println!(
        "game_board after placing `O` on position: {:#?}",
        GameBoard::TOP_LEFT
    );

    let mut game_board2 = GameBoard::new();

    let mut result = Err(String::new());
    for column in 0..=2 {
        result = game_board2.place_piece(&Piece::X, (0, column));
    }

    println!("game_board after all x's placed on top row (illegal game state check not implemented as yet)");
    game_board2.display();

    if let Err(error) = result {
        println!("Could not place X on row 0 of board. Got error: {}", error);
    } else {
        println!("Winner: {:#?}", game_board2.determine_winner());
    }

    game_board2.display();
}
