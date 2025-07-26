
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
            input_from_user::UserCommand::Move(coor) => {
                let coor = (coor.0, coor.1);
                let code = caro_protocol::PlayerCode::PlayerMove(coor);
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                // println!("send: {:?}", new_packet);
                self.requester.write().await.send_request(new_packet).await;
            },
            _ => {

            }
        }
    }
}