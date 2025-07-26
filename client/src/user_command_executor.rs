
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
        let current_state = self.global_state.read().await.get_player_state();
        match command {
            input_from_user::UserCommand::General(command) => {
                self.execute_general_command(command).await;
            },
            input_from_user::UserCommand::Logged(command) => {
                self.execute_logged_command(command).await;
            },
            input_from_user::UserCommand::InRoom(command) => {
                self.execute_inroom_command(command).await;
            },
            input_from_user::UserCommand::InGame(command) => {
                self.execute_ingame_command(command).await;
            },
        }
    }

    async fn execute_general_command(&mut self, command: input_from_user::GeneralCommand) {
        match command {
            input_from_user::GeneralCommand::ExitApplication => {

            },
            input_from_user::GeneralCommand::Invalid => {

            },
        }
    }

    async fn execute_logged_command(&mut self, command: input_from_user::LoggedCommand) {
        match command {
            input_from_user::LoggedCommand::RequestNewRoom(game_rule) => {
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
            input_from_user::LoggedCommand::JoinRoom(rid) => {
                let code = caro_protocol::PlayerCode::JoinRoom(rid);
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                // println!("send: {:?}", new_packet);
                self.requester.write().await.send_request(new_packet).await;
            },
        }
    }

    async fn execute_inroom_command(&mut self, command: input_from_user::InRoomCommand) {
        match command {
            input_from_user::InRoomCommand::LeaveRoom => {

            },
        }
    }

    async fn execute_ingame_command(&mut self, command: input_from_user::InGameCommand) {
        match command {
            input_from_user::InGameCommand::Move(coor) => {
                let coor = (coor.0, coor.1);
                let code = caro_protocol::PlayerCode::PlayerMove(coor);
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                // println!("send: {:?}", new_packet);
                self.requester.write().await.send_request(new_packet).await;
            },
            input_from_user::InGameCommand::Up => {
                
            },
            input_from_user::InGameCommand::Down => {
                
            },
            input_from_user::InGameCommand::Left => {
                
            },
            input_from_user::InGameCommand::Right => {
                
            },
            input_from_user::InGameCommand::Redo => {

            }
            input_from_user::InGameCommand::Undo => {

            },
            input_from_user::InGameCommand::SwitchInputMode => {

            },
            input_from_user::InGameCommand::LeaveRoom => {

            }
        }
    }
}