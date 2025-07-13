use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::{io::{self, AsyncBufReadExt}, sync::Mutex, task::JoinHandle};

pub type Coordinate = (i64, i64);

#[derive(Debug, Clone, Copy)]
pub enum GameRule {
    TicTacToe,
    FourBlockOne,
    FiveBlockTwo,
}

#[derive(Debug, Clone, Copy)]
pub enum UserCommand {
    RequestNewRoom(GameRule),
    JoinRoom(i32),
    Move(Coordinate),
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

pub trait ToUserCommand {
    fn to_user_command(self) -> UserCommand;
}

impl ToUserCommand for &str {
    fn to_user_command(self) -> UserCommand {
        let words: Vec<String>
            = self.to_string()
                .trim()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        match &*words[0] {
            "mkroom" => {
                match &*words[1] {
                    "3" => UserCommand::RequestNewRoom(GameRule::TicTacToe),
                    "4" => UserCommand::RequestNewRoom(GameRule::FourBlockOne),
                    "5" => UserCommand::RequestNewRoom(GameRule::FiveBlockTwo),
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
    }
}

pub type HandleAction = Arc<tokio::sync::Mutex<dyn FnMut(UserCommand) -> BoxFuture<'static, ()> + Send + 'static>>;

pub type InputHandler = JoinHandle<()>;

#[macro_export]
macro_rules! make_input_action {
    ($action:expr) => {
        Arc::new(tokio::sync::Mutex::new($action)) as crate::command_getter::HandleAction
    };
}

pub fn get_input_reader() -> InputReader {
    InputReader {
        reader: io::BufReader::new(io::stdin()).lines(),
        buffer: [0; 1024],
    }
}

pub struct InputReader {
    reader: io::Lines<io::BufReader<io::Stdin>>,
    buffer: [u8; 1024],
}

impl InputReader {
    async fn get_input_line(&mut self) -> String {
        if let Some(line) = self.reader.next_line().await.unwrap() {
            line
        } else {
            "".to_string()
        }
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

    pub async fn handling_input(target: Arc<Mutex<CommandGetter>>) -> InputHandler {
        let target_clone = target.clone();
        tokio::spawn(
            async move {
                let target = target_clone.clone();
                loop {
                    let input_line = target.lock().await.input_reader.get_input_line().await;
                    let cmd = input_line.to_user_command();
                    tokio::spawn(target.lock().await.action.lock().await(cmd));
                }
            }
        )
    }

    pub fn stop_handling_input(handler: InputHandler) {
        handler.abort();
    }
}
