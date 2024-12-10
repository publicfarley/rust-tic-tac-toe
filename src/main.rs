mod game_model;

use crate::game_model::{CellState, GameBoard};

fn main() {
    for state in CellState::all_cases() {
        println!("Hello, cell state! {state:#?}");
    }

    let game_board: GameBoard = Default::default();
    println!("Hello game board: {game_board:#?}");

    println!("{:#?}", game_board[0][0]);
    println!("{:#?}", game_board[0][0]);
}
