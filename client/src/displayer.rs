use crate::caro_protocol;
use std::cmp::min;

const SCREEN_WIDTH: usize = 10;
const SCREEN_HEIGHT: usize = 10;

pub fn print_caro_board(board: Vec<caro_protocol::Row>) {
    if board.len() == 0 || board[0].len() == 0 {
        return;
    }
    let max_height = min(board.len(), SCREEN_HEIGHT);
    let max_width = min(board[0].len(), SCREEN_WIDTH);
    for row in &board[..max_height] {
        print!("[");
        for tile in &row[..max_width] {
            match tile {
                caro_protocol::TileState::Empty => print!(" ."),
                caro_protocol::TileState::Player1 => print!(" X"),
                caro_protocol::TileState::Player2 => print!(" O"),
            }
        }
        println!("]");
    }
}

pub fn print_caro_context(context: caro_protocol::GameContext) {
    if context.board_height <= 0 || context.board_width <= 0 {
        return;
    }

    let player1_move_history = context.player1_move_history;
    let player1_occupied = move |latitude: i64, longtitude: i64| -> bool {
        let target = player1_move_history.iter().find(|(llatitude, llongtitude)| {
            latitude == *llatitude && longtitude == *llongtitude
        });
        if let Some(_coor) = target {
            true
        } else {
            false
        }
    };
    let player2_move_history = context.player2_move_history;
    let player2_occupied = move |latitude: i64, longtitude: i64| -> bool {
        let target = player2_move_history.iter().find(|(llatitude, llongtitude)| {
            latitude == *llatitude && longtitude == *llongtitude
        });
        if let Some(_coor) = target {
            true
        } else {
            false
        }
    };

    let max_height = min(context.board_height, SCREEN_HEIGHT);
    let max_width = min(context.board_width, SCREEN_WIDTH);
    println!("======================");
    for latitude in 0..max_height {
        print!("[");
        for longtitude in 0..max_width {
            if player1_occupied(latitude as i64, longtitude as i64) {
                print!("X ");
            } else if player2_occupied(latitude as i64, longtitude as i64) {
                print!("O ");
            } else {
                print!(". ");
            }
        }
        println!("]");
    }
    println!("======================");
}

pub fn print_notification(message: &str) {
    println!("{}", message);
}