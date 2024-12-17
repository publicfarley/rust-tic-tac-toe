mod game_model;

use crate::game_model::{GameBoard, Piece};

fn main() {
    let mut game_board = GameBoard::new();
    println!("Empty game board:");
    game_board.display();
    println!();

    _ = game_board.place_piece(&Piece::O, GameBoard::TOP_LEFT);
    println!(
        "game_board after placing `O` on position: {:#?}",
        GameBoard::TOP_LEFT
    );
    game_board.display();
    println!();

    let mut game_board2 = GameBoard::new();

    let result = game_board2
        .place_piece(&Piece::X, GameBoard::TOP_LEFT)
        .and_then(|_| game_board2.place_piece(&Piece::X, GameBoard::TOP_CENTER))
        .and_then(|_| game_board2.place_piece(&Piece::X, GameBoard::TOP_RIGHT));

    println!("All Xs on row 0 (Illegal I know)");
    game_board2.display();
    println!();

    if let Err(error) = result {
        println!("Could not place X on row 0 of board. Got error: {}", error);
    } else {
        match game_board2.determine_winner() {
            Some(piece) => println!("Winner: {:#?}", piece),
            None => println!("No winner yet"),
        }
    }

    println!();
}
