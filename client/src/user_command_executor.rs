
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{caro_protocol, client_endpoint, global_state, input_from_user, output_to_user};

pub struct CommandExecutor {
    global_state: Arc<RwLock<global_state::GlobalState>>,
    screen_manager: Arc<RwLock<output_to_user::ScreenManager>>,
    requester: Arc<RwLock<client_endpoint::Requester>>,
}

impl CommandExecutor {
    pub fn new(global_state: Arc<RwLock<global_state::GlobalState>>,
                screen_manager: Arc<RwLock<output_to_user::ScreenManager>>,
                requester: Arc<RwLock<client_endpoint::Requester>>) -> Self {
        Self {
            global_state,
            screen_manager,
            requester,
        }
    }

    pub async fn execute_command(&mut self, command: input_from_user::UserCommand) {
        // global request (regardless of owner's state)
        self.execute_general_command(command).await;

        let current_state = self.global_state.read().await.get_player_state();
        match current_state {
            caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected) => {
                self.execute_logged_command(command).await;
            },
            caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected) => {
                self.execute_inroom_command(command).await;
            },
            caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected) => {
                self.execute_ingame_command(command).await;
            },
            _ => {
                // do not process other commands
            }
        }
    }

    async fn execute_general_command(&mut self, command: input_from_user::UserCommand) {
        match command {
            input_from_user::UserCommand::ExitApplication => {

            },
            input_from_user::UserCommand::Invalid => {

            },
            _ => {
                // do not process other commands
            }
        }
    }

    async fn execute_logged_command(&mut self, command: input_from_user::UserCommand) {
        match command {
            input_from_user::UserCommand::RequestNewRoom(game_rule) => {
                match game_rule {
                    caro_protocol::GameRule::TicTacToe => {
                        let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::TicTacToe);
                        let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                        // println!("send: {:?}", new_packet);
                        self.requester.write().await.send_request(new_packet).await;
                    },
                    caro_protocol::GameRule::FourBlockOne => {
                        let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::FourBlockOne);
                        let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                        // println!("send: {:?}", new_packet);
                        self.requester.write().await.send_request(new_packet).await;
                    },
                    caro_protocol::GameRule::FiveBlockTwo => {
                        let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::FiveBlockTwo);
                        let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                        // println!("send: {:?}", new_packet);
                        self.requester.write().await.send_request(new_packet).await;
                    },
                }
            },
            input_from_user::UserCommand::JoinRoom(rid) => {
                let code = caro_protocol::PlayerCode::JoinRoom(rid);
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                // println!("send: {:?}", new_packet);
                self.requester.write().await.send_request(new_packet).await;
            },
            input_from_user::UserCommand::LeaveRoom => {

            },
            _ => {
                // do not process other commands
            }
        }
    }

    async fn execute_inroom_command(&mut self, command: input_from_user::UserCommand) {
        match command {
            input_from_user::UserCommand::LeaveRoom => {

            },
            _ => {
                // do not process other commands
            }
        }
    }

    async fn execute_ingame_command(&mut self, command: input_from_user::UserCommand) {
        match command {
            input_from_user::UserCommand::Move(coor) => {
                let coor = (coor.0, coor.1);
                let code = caro_protocol::PlayerCode::PlayerMove(coor);
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                // println!("send: {:?}", new_packet);
                self.requester.write().await.send_request(new_packet).await;
            },
            input_from_user::UserCommand::Up => {
                
            },
            input_from_user::UserCommand::Down => {
                
            },
            input_from_user::UserCommand::Left => {
                
            },
            input_from_user::UserCommand::Right => {
                
            },
            input_from_user::UserCommand::Redo => {

            }
            input_from_user::UserCommand::Undo => {

            },
            input_from_user::UserCommand::SwitchInputMode => {

            },
            _ => {
                // do not process other commands
            }
        }
    }
}