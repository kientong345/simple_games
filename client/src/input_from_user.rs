use std::{process::exit, sync::Arc};

use futures::future::BoxFuture;
use tokio::{sync::RwLock, task::JoinHandle};
use crate::{caro_protocol, input_from_user::command_parser::ToUserCommand};
use caro_console;

pub mod command_parser;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneralCommand {
    ExitApplication,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoggedCommand {
    RequestNewRoom(caro_protocol::GameRule),
    JoinRoom(caro_protocol::RoomId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InRoomCommand {
    LeaveRoom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InGameCommand {
    Move(caro_protocol::Coordinate),
    Up,
    Down,
    Left,
    Right,
    Enter,
    Undo,
    Redo,
    SwitchInputMode,
    LeaveRoom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserCommand {
    General(GeneralCommand),
    Logged(LoggedCommand),
    InRoom(InRoomCommand),
    InGame(InGameCommand),
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
                    // if let caro_console::input::InputType::Key(_key_type) = input_line {
                    //     exit(1);
                    // }
                    let cmd = input_line.to_user_command();
                    // tokio::spawn(target.read().await.action.write().await(cmd));
                    target.read().await.action.write().await(cmd).await;
                }
            }
        )
    }

    pub fn stop_handling_input(handler: InputHandler) {
        handler.abort();
    }
}
