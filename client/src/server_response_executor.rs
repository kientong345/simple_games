// most logic would be moved to here

use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

use crate::{caro_protocol, client_endpoint, global_state, output_to_user};

pub struct ResponseExecutor {
    global_state: Arc<RwLock<global_state::GolbalState>>,
    screen_manager: Arc<Mutex<output_to_user::ScreenManager>>,
    requester: Arc<Mutex<client_endpoint::Requester>>,
}

impl ResponseExecutor {
    pub fn new(global_state: Arc<RwLock<global_state::GolbalState>>,
                screen_manager: Arc<Mutex<output_to_user::ScreenManager>>,
                requester: Arc<Mutex<client_endpoint::Requester>>,) -> Self {
        Self {
            global_state,
            screen_manager,
            requester,
        }
    }

    pub async fn execute_response(&mut self, code: caro_protocol::ServerCode) {
        match code {
            caro_protocol::ServerCode::AreYouAlive => {
                let code = caro_protocol::PlayerCode::IAmAlive;
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                self.requester.lock().await.send_request(new_packet).await;
            },
            caro_protocol::ServerCode::JoinedRoomAsPlayer1(rid) => {
                let conn_state = self.global_state.read().await.get_connection_state();
                match conn_state {
                    caro_protocol::ConnectState::Connected => {
                        self.global_state.write().await.set_player_state(caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected));
                        self.global_state.write().await.set_current_rid(rid);
                        self.screen_manager.lock().await.clean();
                        self.screen_manager.lock().await.set_player_order(caro_protocol::PlayerOrder::Player1);
                        self.screen_manager.lock().await.update().await;
                        let log_content = "JoinedRoomAsPlayer1 in room".to_string() + &rid.to_string();
                        self.screen_manager.lock().await.log(log_content).await;
                        self.screen_manager.lock().await.enable_prompt_mode().await;
                    },
                    caro_protocol::ConnectState::Disconnected => {
                    },
                }
            },
            caro_protocol::ServerCode::JoinedRoomAsPlayer2(rid) => {
                let conn_state = self.global_state.read().await.get_connection_state();
                match conn_state {
                    caro_protocol::ConnectState::Connected => {
                        self.global_state.write().await.set_player_state(caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected));
                        self.global_state.write().await.set_current_rid(rid);
                        self.screen_manager.lock().await.clean();
                        self.screen_manager.lock().await.set_player_order(caro_protocol::PlayerOrder::Player2);
                        self.screen_manager.lock().await.update().await;
                        let log_content = "JoinedRoomAsPlayer2".to_string() + &rid.to_string();
                        self.screen_manager.lock().await.log(log_content).await;
                        self.screen_manager.lock().await.enable_prompt_mode().await;
                    },
                    caro_protocol::ConnectState::Disconnected => {

                    },
                }
            },
            caro_protocol::ServerCode::FailedToCreateRoom => {

            },
            caro_protocol::ServerCode::FailedToJoinRoom(rid) => {

            },
            caro_protocol::ServerCode::YourRoomIsFull(rid) => {
                let conn_state = self.global_state.read().await.get_connection_state();
                match conn_state {
                    caro_protocol::ConnectState::Connected => {
                        self.global_state.write().await.set_player_state(caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected));
                        self.global_state.write().await.set_current_rid(rid);
                        self.screen_manager.lock().await.clean();
                        self.screen_manager.lock().await.update().await;
                        self.screen_manager.lock().await.log("game is ready!".to_string()).await;
                        self.screen_manager.lock().await.enable_prompt_mode().await;
                    },
                    caro_protocol::ConnectState::Disconnected => {

                    },
                }
            },
            caro_protocol::ServerCode::MoveSuccess => {

            },
            caro_protocol::ServerCode::MoveUnsuccess => {

            },
            caro_protocol::ServerCode::Context(game_context) => {
                // screen_manager::print_caro_context(game_context);
                self.screen_manager.lock().await.update_game_context(&game_context);
                self.screen_manager.lock().await.update_board_only().await;
            },
            _ => (),
        }
    }
}