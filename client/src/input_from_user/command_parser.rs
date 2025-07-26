use crate::{caro_protocol, input_from_user::{GeneralCommand, InGameCommand, LoggedCommand, UserCommand}};

pub trait ToUserCommand {
    fn to_user_command(self) -> UserCommand;
}

impl ToUserCommand for caro_console::input::InputType {
    fn to_user_command(self) -> UserCommand {
        match self {
            caro_console::input::InputType::Text(line) => {
                let words: Vec<String>
                = line.to_string()
                    .trim()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
                match &*words[0] {
                    "mkroom" => {
                        match &*words[1] {
                            "3" => UserCommand::Logged(LoggedCommand::RequestNewRoom(caro_protocol::GameRule::TicTacToe)),
                            "4" => UserCommand::Logged(LoggedCommand::RequestNewRoom(caro_protocol::GameRule::FourBlockOne)),
                            "5" => UserCommand::Logged(LoggedCommand::RequestNewRoom(caro_protocol::GameRule::FiveBlockTwo)),
                            _ => UserCommand::General(GeneralCommand::Invalid),
                        }
                    },
                    "cdroom" => {
                        let rid = words[1].parse().unwrap();
                        UserCommand::Logged(LoggedCommand::JoinRoom(rid))
                    },
                    "move" => {
                        let latitude = words[1].parse().unwrap();
                        let longtitude = words[2].parse().unwrap();
                        UserCommand::InGame(InGameCommand::Move((latitude, longtitude)))
                    },
                    _ => UserCommand::General(GeneralCommand::Invalid),
                }
            },
            caro_console::input::InputType::Key(key) => {
                match key {
                    caro_console::input::KeyType::Up => UserCommand::InGame(InGameCommand::Up),
                    caro_console::input::KeyType::Down => UserCommand::InGame(InGameCommand::Down),
                    caro_console::input::KeyType::Left => UserCommand::InGame(InGameCommand::Left),
                    caro_console::input::KeyType::Right => UserCommand::InGame(InGameCommand::Right),
                    caro_console::input::KeyType::Esc => UserCommand::InGame(InGameCommand::SwitchInputMode),
                    caro_console::input::KeyType::Invalid => UserCommand::General(GeneralCommand::Invalid),
                }
            },
        }
    }
}