use crate::caro_protocol;
use std::{cmp::min, io};

const SCREEN_WIDTH: usize = 20;
const SCREEN_HEIGHT: usize = 20;

pub fn get_command() -> caro_protocol::PlayerCode {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let words: Vec<String> = input
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    match &*words[0] {
        "mkroom" => {
            match &*words[1] {
                "3" => caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::TicTacToe),
                "4" => caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::FourBlockOne),
                "5" => caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::FiveBlockTwo),
                _ => caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::TicTacToe),
            }
        },
        "cdroom" => {
            let rid = words[1].parse().unwrap();
            caro_protocol::PlayerCode::JoinRoomAsPlayer2(rid)
        },
        "move1" => {
            let latitude = words[1].parse().unwrap();
            let longtitude = words[2].parse().unwrap();
            caro_protocol::PlayerCode::Player1Move((latitude, longtitude))
        },
        "move2" => {
            let latitude = words[1].parse().unwrap();
            let longtitude = words[2].parse().unwrap();
            caro_protocol::PlayerCode::Player2Move((latitude, longtitude))
        },
        _ => caro_protocol::PlayerCode::Player1RequestContext, // dummy
    }
}

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

pub fn print_notification(message: &str) {
    println!("{}", message);
}