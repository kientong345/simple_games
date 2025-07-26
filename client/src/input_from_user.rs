use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::{sync::RwLock, task::JoinHandle};
use crate::caro_protocol;
use caro_console;

#[derive(Debug, Clone, Copy)]
pub enum UserCommand {
    RequestNewRoom(caro_protocol::GameRule),
    JoinRoom(caro_protocol::RoomId),
    LeaveRoom,
    ExitApplication,
    Move(caro_protocol::Coordinate),
    Up,
    Down,
    Left,
    Right,
    Undo,
    Redo,
    SwitchInputMode,
    Invalid,
}

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
                            "3" => UserCommand::RequestNewRoom(caro_protocol::GameRule::TicTacToe),
                            "4" => UserCommand::RequestNewRoom(caro_protocol::GameRule::FourBlockOne),
                            "5" => UserCommand::RequestNewRoom(caro_protocol::GameRule::FiveBlockTwo),
                            _ => UserCommand::Invalid,
                        }
                    },
                    "cdroom" => {
                        let rid = words[1].parse().unwrap();
                        UserCommand::JoinRoom(rid)
                    },
                    "move" => {
                        let latitude = words[1].parse().unwrap();
                        let longtitude = words[2].parse().unwrap();
                        UserCommand::Move((latitude, longtitude))
                    },
                    _ => UserCommand::Invalid,
                }
            },
            caro_console::input::InputType::Key(key) => {
                UserCommand::Invalid
            },
        }
    }
}

pub type HandleAction = Arc<tokio::sync::RwLock<dyn FnMut(UserCommand) -> BoxFuture<'static, ()> + Send + Sync + 'static>>;

pub type InputHandler = JoinHandle<()>;

#[macro_export]
macro_rules! make_input_action {
    ($action:expr) => {
        Arc::new(tokio::sync::RwLock::new($action)) as crate::input_from_user::HandleAction
    };
}

pub fn get_input_reader() -> InputReader {
    InputReader {

    }
}

pub struct InputReader {

}

impl InputReader {
    async fn get_input(&mut self) -> caro_console::input::InputType {
        caro_console::input::get_user_input().await
    }
}

pub struct CommandGetter {
    input_reader: InputReader,
    action: HandleAction,
}

impl CommandGetter {
    pub fn new(input_reader: InputReader) -> Self {
        let action = make_input_action!(|_msg: UserCommand| {
            let future = async move {
            };
            Box::pin(future) as BoxFuture<'static, ()>
        });
        Self {
            input_reader,
            action,
        }
    }

    pub fn set_action_on_input(&mut self, action: HandleAction) {
        self.action = action;
    }

    pub fn get_action_on_input(&self) -> HandleAction {
        self.action.clone()
    }

    pub async fn handling_input(target: Arc<RwLock<CommandGetter>>) -> InputHandler {
        let target_clone = target.clone();
        tokio::spawn(
            async move {
                let target = target_clone.clone();
                loop {
                    let input_line = target.write().await.input_reader.get_input().await;
                    let cmd = input_line.to_user_command();
                    tokio::spawn(target.read().await.action.write().await(cmd));
                }
            }
        )
    }

    pub fn stop_handling_input(handler: InputHandler) {
        handler.abort();
    }
}
