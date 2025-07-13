use crate::caro_protocol;
use std::io;

struct CommandGetter {

}

impl CommandGetter {
    
}

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
            caro_protocol::PlayerCode::JoinRoom(rid)
        },
        "move" => {
            let latitude = words[1].parse().unwrap();
            let longtitude = words[2].parse().unwrap();
            caro_protocol::PlayerCode::PlayerMove((latitude, longtitude))
        },
        _ => caro_protocol::PlayerCode::PlayerRequestContext, // dummy
    }
}