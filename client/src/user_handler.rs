use crate::caro_protocol;
use std::io;

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
        _ => caro_protocol::PlayerCode::Player1Leave, // dummy
    }
}

pub fn print_caro_board(board: Vec<caro_protocol::Row>) {
    todo!()
}

pub fn print_notification(message: &str) {
    println!("{}", message);
}