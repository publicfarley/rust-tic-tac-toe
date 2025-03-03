mod game_model;
use crate::game_model::{CellState, GameBoard, Player};

use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let usage_string = "Usage: cargo run -- {text|gui}";

    // The first argument (args[0]) is always the program name
    // Actual arguments start from args[1]
    if args.len() <= 1 {
        println!("{usage_string}");
        return;
    }

    let mode = args[1].as_str();
    match mode {
        "text" => text_main(),
        "gui" => gui_main(),
        _ => {
            println!("Wrong mode. Mode must be 'text' or 'gui', but got '{mode}` instead.");
            println!("{usage_string}");
        }
    }
}

fn text_main() {
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

    let game_end_message = game_loop(&mut game_board);
    println!("{game_end_message}");
    println!("Thanks, play again soon!");
}

fn game_loop(game_board: &mut GameBoard) -> String {
    let mut game_end_message = String::new();

    while game_end_message.is_empty() {
        let next_player_up = game_board.player_for_id(game_board.next_up);

        let turn_result = match next_player_up {
            Player::Computer(_) => {
                display_spinner_with_message("The computer is thinking...");
                execute_computer_turn(game_board)
            }

            Player::Human(_) => execute_human_turn(game_board),
        };

        println!("\nGame board:\n{game_board}");

        match turn_result {
            Ok(()) => {
                let winner = game_board.determine_winning_player();

                match winner {
                    Some(player) => match player {
                        Player::Human(piece) => {
                            game_end_message = format!("\n✨{piece}✨ You won! 🥇");
                        }
                        Player::Computer(piece) => {
                            game_end_message = format!("\n✨{piece}✨ The computer won! 🥇");
                        }
                    },

                    None => {
                        if game_board.is_board_full() {
                            game_end_message =
                                "\nThis game results in a draw. The board is full.".to_string();
                        }
                    }
                }
            }

            Err(error) => {
                if error == "exit" {
                    game_end_message = "\nExiting the game".to_string();
                } else {
                    // Output the error condition and continue looping
                    println!("\nError: {error:?}");
                }
            }
        }
    }

    game_end_message
}

fn execute_computer_turn(game_board: &mut GameBoard) -> Result<(), String> {
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

    let iterations = 0..32;
    for i in iterations {
        // Use modulus to cycle through the spinner characters
        let spinner_value = spinner_chars[i % spinner_chars.len()];

        display_character(message, spinner_value);

        // Wait for 100ms before updating the spinner
        thread::sleep(Duration::from_millis(100));
    }

    let clear_message = ' '.to_string().repeat(message.len());
    display_character(&clear_message, ' ');
}

use eframe::egui::{self, Response};

struct TicTacToeApp {
    game_end_message: String,
    game_board: GameBoard,
}

impl TicTacToeApp {
    fn new() -> Self {
        Self {
            game_end_message: String::new(),
            game_board: GameBoard::new(),
        }
    }
}

fn gui_main() {
    let options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        "Tic Tac Toe",
        options,
        Box::new(|_cc| Box::new(TicTacToeApp::new())),
    );
}

impl eframe::App for TicTacToeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.game_end_message.is_empty() {
            self.game_end_message = update_ui(self, ctx);
            return;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(self.game_end_message.to_string())
                        .size(24.0)
                        .strong()
                        .color(egui::Color32::from_rgb(100, 100, 255)),
                );
            });
        });
    }
}

fn update_ui(app: &mut TicTacToeApp, ctx: &egui::Context) -> String {
    let mut turn_result: Result<(), String> = Ok(());

    egui::CentralPanel::default().show(ctx, |ui| {
        // Define the size of the board
        let board_size = 300.0;
        let cell_size = board_size / 3.0;

        // Create a painter to draw the grid and marks
        let (response, painter) =
            ui.allocate_painter(egui::Vec2::splat(board_size), egui::Sense::click());

        draw_grid_lines(&painter, cell_size, board_size);

        draw_board_contents(&app.game_board, &painter, cell_size);

        turn_result = if app.game_board.is_computers_turn() {
            execute_computer_turn(&mut app.game_board)
        } else if response.clicked() {
            app.game_board
                .update_board_based_on_response(&response, cell_size)
        } else {
            Ok(())
        }
    });

    handle_turn_result(&turn_result, &app.game_board)
}

fn handle_turn_result(turn_result: &Result<(), String>, game_board: &GameBoard) -> String {
    let mut game_end_message: String = String::new();

    match turn_result {
        Ok(()) => {
            let winner = game_board.determine_winning_player();

            match winner {
                Some(player) => match player {
                    Player::Human(piece) => {
                        game_end_message = format!("\n✨{piece}✨ You won! 🥇");
                    }
                    Player::Computer(piece) => {
                        game_end_message = format!("\n✨{piece}✨ The computer won! 🥇");
                    }
                },

                None => {
                    if game_board.is_board_full() {
                        game_end_message =
                            "\nThis game results in a draw. The board is full.".to_string();
                    }
                }
            }
        }

        Err(error) => {
            if error == "exit" {
                game_end_message = "\nExiting the game".to_string();
            } else {
                // Output the error condition and continue looping
                game_end_message = String::new();
                println!("\nError: {error:?}");
            }
        }
    };

    game_end_message
}

fn draw_grid_lines(painter: &egui::Painter, cell_size: f32, board_size: f32) {
    for i in 1i16..3 {
        let offset = f32::from(i) * cell_size;
        // Vertical lines
        painter.line_segment(
            [egui::pos2(offset, 0.0), egui::pos2(offset, board_size)],
            (2.0, egui::Color32::BLACK),
        );
        // Horizontal lines
        painter.line_segment(
            [egui::pos2(0.0, offset), egui::pos2(board_size, offset)],
            (2.0, egui::Color32::BLACK),
        );
    }
}

fn draw_x(painter: &egui::Painter, center: egui::Pos2) {
    painter.line_segment(
        [
            egui::pos2(center.x - 20.0, center.y - 20.0),
            egui::pos2(center.x + 20.0, center.y + 20.0),
        ],
        (2.0, egui::Color32::RED),
    );
    painter.line_segment(
        [
            egui::pos2(center.x + 20.0, center.y - 20.0),
            egui::pos2(center.x - 20.0, center.y + 20.0),
        ],
        (2.0, egui::Color32::RED),
    );
}

fn draw_o(painter: &egui::Painter, center: egui::Pos2) {
    painter.circle_stroke(center, 20.0, (2.0, egui::Color32::BLUE));
}

fn draw_board_contents(game_board: &GameBoard, painter: &egui::Painter, cell_size: f32) {
    // Draw X and O marks on the board
    for row in 0..3 {
        for col in 0..3 {
            let row_id: usize = match row {
                0 => 1,
                1 => 4,
                2 => 7,
                _ => 0,
            };
            let position: usize = row_id + col;
            let Some(mark) = game_board.get_cell_at_position(position) else {
                break;
            };

            match mark {
                CellState::Empty => (),
                CellState::Occupied(piece) => {
                    let Some(column_position) = usize_to_f32(col) else {
                        break;
                    };

                    let Some(row_position) = usize_to_f32(row) else {
                        break;
                    };

                    let center = egui::pos2(
                        column_position.mul_add(cell_size, cell_size / 2.0),
                        row_position.mul_add(cell_size, cell_size / 2.0),
                    );

                    match piece {
                        game_model::Piece::X => {
                            draw_x(painter, center);
                        }
                        game_model::Piece::O => {
                            draw_o(painter, center);
                        }
                    }
                }
            }
        }
    }
}

impl GameBoard {
    fn update_board_based_on_response(
        &mut self,
        response: &Response,
        cell_size: f32,
    ) -> Result<(), String> {
        let Some(pos) = response.hover_pos() else {
            return Ok(());
        };

        let Some(col) = f32_to_usize((pos.x / cell_size).floor()) else {
            return Ok(());
        };

        let Some(row) = f32_to_usize((pos.y / cell_size).floor()) else {
            return Ok(());
        };

        let row_id: usize = match row {
            0 => 1,
            1 => 4,
            2 => 7,
            _ => 0,
        };

        let position = row_id + col;

        if position <= Self::POSITIONS.count()
            && self.get_cell_at_position(position) == Some(&CellState::Empty)
        {
            self.play_next_up_at_position(position)
        } else {
            Ok(())
        }
    }
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
const fn usize_to_f32(usize_value: usize) -> Option<f32> {
    let float_value = usize_value as f32;
    if float_value as usize == usize_value {
        Some(float_value)
    } else {
        None // Loss in precision occurred.
    }
}

#[allow(clippy::cast_precision_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_sign_loss)]
#[allow(clippy::float_cmp)]
const fn f32_to_usize(f32_value: f32) -> Option<usize> {
    let usize_value = f32_value as usize;
    if usize_value as f32 == f32_value {
        Some(usize_value)
    } else {
        None // Loss in precision occurred.
    }
}
