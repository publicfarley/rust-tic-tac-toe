mod game_model;
use crate::game_model::{GameBoard, Player};

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let mut game_board = GameBoard::new();
    println!("\nWelcome to Rusty 🦀 Tic Tac Toe:\n{game_board}");

    let first_up = game_board.next_up;
    let first_up_player = game_board.player_for_id(first_up);

    match first_up_player {
        Player::Human(piece) => {
            println!("\nYou are first up for this game. You have been assigned piece: {piece}");
        }

        Player::Computer(piece) => println!(
            "\nFirst up for this game is the {}. They have been assigned piece: {piece}",
            first_up_player.name()
        ),
    };

    match game_loop(&mut game_board) {
        Ok(message) => {
            println!("{message}\n");
            println!("Thanks, play again soon!");
        }
        Err(error) => println!("{error:#?}"),
    }
}

fn game_loop(game_board: &mut GameBoard) -> Result<String, String> {
    loop {
        let next_player_up = game_board.player_for_id(game_board.next_up);

        let turn_result = match next_player_up {
            Player::Computer(_) => execute_computer_turn(game_board),
            Player::Human(_) => execute_human_turn(game_board),
        };

        println!("\nGame board:\n{game_board}");

        match turn_result {
            Ok(()) => {
                let winner = game_board.determine_winning_player();
                match winner {
                    Some(player) => match player {
                        Player::Human(piece) => break Ok(format!("\n✨{piece}✨ You won! 🥇")),
                        Player::Computer(piece) => {
                            break Ok(format!("\n✨{piece}✨ The computer won! 🥇"))
                        }
                    },

                    None => {
                        if game_board.is_board_full() {
                            break Ok(
                                "\nThis game results in a draw. The board is full.".to_string()
                            );
                        }
                    }
                }
            }

            Err(error) => {
                if error == "exit" {
                    break Ok("\nExiting the game".to_string());
                }

                // Continue looping
                println!("\nError: {error:?}");
            }
        }
    }
}

fn execute_computer_turn(game_board: &mut GameBoard) -> Result<(), String> {
    game_board.get_random_available_position().map_or_else(
        || Err("No available positions".to_string()),
        |position| {
            display_spinner_with_message("The computer is thinking...");
            let piece = game_board.player_for_id(game_board.next_up).piece();
            println!("\nThe computer played {piece} in position: {position}");
            game_board.play_next_up_at_position(position)
        },
    )
}

fn execute_human_turn(game_board: &mut GameBoard) -> Result<(), String> {
    // Prompt the user
    print!(
        "\nEnter a number from the available positions: {:?} or (q/Q to quit). ",
        game_board.get_available_positions()
    );

    if io::stdout().flush().is_err() {
        return Err(String::from("Failed to flush stdout"));
    }

    // Read user input
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Err(String::from("Failed to read line"));
    }

    let input = input.trim();

    if input.eq_ignore_ascii_case("q") {
        println!("You entered 'q' or 'Q'. Exiting...");
        return Err(String::from("exit"));
    }

    // Attempt to parse the input as an usize
    input.parse::<usize>().map_or_else(
        |_| Err("Invalid input. Please enter a valid number or type 'q/Q' to quit.".to_string()),
        |position| {
            let piece = game_board.player_for_id(game_board.next_up).piece();
            println!("\nYou played {piece} in position: {position}");
            game_board.play_next_up_at_position(position)
        },
    )
}

fn display_spinner_with_message(message: &str) {
    let spinner_chars = ['|', '/', '-', '\\'];

    let display_character = |message: &str, character: char| {
        // Print the spinner and flush to show it immediately
        print!("\r{message} {character}");
        if io::stdout().flush().is_err() {
            println!("Failed to flush stdout");
        }
    };

    for i in 0..32 {
        // Use modulus to cycle through the spinner characters
        let spinner_value = spinner_chars[i % spinner_chars.len()];

        display_character(message, spinner_value);

        // Wait for 100ms before updating the spinner
        thread::sleep(Duration::from_millis(100));
    }

    let clear_message = ' '.to_string().repeat(message.len());
    display_character(&clear_message, ' ');
}
