mod game_model;
use crate::game_model::{GameBoard, Piece};
use game_model::Coordinate;

fn main() {
    let mut game_board = GameBoard::new();
    println!("Empty game board:\n{game_board}");

    let first_up = game_board.next_up;
    println!("\nFirst up today: {first_up:?}");

    _ = game_board.place_piece(first_up, &GameBoard::TOP_LEFT);
    println!(
        "game_board after placing `{first_up}` on position: {:#?}",
        GameBoard::TOP_LEFT
    );
    println!("{game_board}");

    if let Some(middle_coordinate) = Coordinate::new(1, 1) {
        println!(
            "Cell at {middle_coordinate:?} is {:#?}",
            game_board.get_cell(&middle_coordinate)
        );
    }

    println!("\nNew game:");
    let mut game_board2 = GameBoard::new_with_first_up(Piece::X);
    let result = game_board2
        .place_piece(Piece::X, &GameBoard::TOP_LEFT)
        .and_then(|()| game_board2.place_piece(Piece::O, &GameBoard::MIDDLE_CENTER))
        .and_then(|()| game_board2.place_piece(Piece::X, &GameBoard::TOP_CENTER))
        .and_then(|()| game_board2.place_piece(Piece::O, &GameBoard::BOTTOM_CENTER))
        .and_then(|()| game_board2.place_piece(Piece::X, &GameBoard::TOP_RIGHT));

    println!("{game_board2}");

    if let Err(error) = result {
        println!("Could not place piece. Got error: {error}");
    } else {
        match game_board2.determine_winner() {
            Some(piece) => println!("Winner: {piece:#?}"),
            None => println!("No winner yet"),
        }
    }
}
