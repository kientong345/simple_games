// most logic would be moved to here

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{caro_protocol, client_endpoint, global_state, output_to_user};

pub struct ResponseExecutor {
    global_state: Arc<RwLock<global_state::GlobalState>>,
    screen_manager: Arc<RwLock<output_to_user::ScreenManager>>,
    requester: Arc<RwLock<client_endpoint::Requester>>,
}

impl ResponseExecutor {
    pub fn new(global_state: Arc<RwLock<global_state::GlobalState>>,
                screen_manager: Arc<RwLock<output_to_user::ScreenManager>>,
                requester: Arc<RwLock<client_endpoint::Requester>>) -> Self {
        Self {
            global_state,
            screen_manager,
            requester,
        }
    }

    pub async fn execute_response(&mut self, response_type: caro_protocol::ServerCode) {
        let current_state = self.global_state.read().await.get_player_state();
        match response_type {
            caro_protocol::ServerCode::General(code) => {
                self.execute_general_response(code).await;
            },
            caro_protocol::ServerCode::Logged(code) => {
                if current_state == caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected) {
                    self.execute_logged_response(code).await;
                }
            },
            caro_protocol::ServerCode::InRoom(code) => {
                if current_state == caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected) {
                    self.execute_inroom_response(code).await;
                }
            },
            caro_protocol::ServerCode::InGame(code) => {
                if current_state == caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected) {
                    self.execute_ingame_response(code).await;
                }
            },
        }
    }

    async fn execute_general_response(&mut self, code: caro_protocol::GeneralResponse) {
        match code {
            caro_protocol::GeneralResponse::State(your_state) => {

            },
            caro_protocol::GeneralResponse::AreYouAlive => {
                let code = caro_protocol::PlayerCode::General(caro_protocol::GeneralRequest::IAmAlive);
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                self.requester.write().await.send_request(new_packet).await;
            },
        }
    }

    async fn execute_logged_response(&mut self, code: caro_protocol::LoggedResponse) {
        match code {
            caro_protocol::LoggedResponse::JoinedRoomAsPlayer1(rid) => {
                let conn_state = self.global_state.read().await.get_connection_state();
                match conn_state {
                    caro_protocol::ConnectState::Connected => {
                        self.global_state.write().await.set_player_state(caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected));
                        self.global_state.write().await.set_current_rid(rid);
                        self.screen_manager.write().await.clean();
                        self.screen_manager.write().await.set_player_order(caro_protocol::PlayerOrder::Player1);
                        self.screen_manager.write().await.update().await;
                        let log_content = "JoinedRoomAsPlayer1 in room".to_string() + &rid.to_string();
                        self.screen_manager.write().await.log(log_content).await;
                        self.screen_manager.write().await.enable_prompt_mode().await;
                    },
                    caro_protocol::ConnectState::Disconnected => {
                    },
                }
            },
            caro_protocol::LoggedResponse::JoinedRoomAsPlayer2(rid) => {
                let conn_state = self.global_state.read().await.get_connection_state();
                match conn_state {
                    caro_protocol::ConnectState::Connected => {
                        self.global_state.write().await.set_player_state(caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected));
                        self.global_state.write().await.set_current_rid(rid);
                        self.screen_manager.write().await.clean();
                        self.screen_manager.write().await.set_player_order(caro_protocol::PlayerOrder::Player2);
                        self.screen_manager.write().await.update().await;
                        let log_content = "JoinedRoomAsPlayer2".to_string() + &rid.to_string();
                        self.screen_manager.write().await.log(log_content).await;
                        self.screen_manager.write().await.enable_prompt_mode().await;
                    },
                    caro_protocol::ConnectState::Disconnected => {

                    },
                }
            },
            caro_protocol::LoggedResponse::FailedToCreateRoom => {

            },
            caro_protocol::LoggedResponse::FailedToJoinRoom(rid) => {

            },
        }
    }

    async fn execute_inroom_response(&mut self, code: caro_protocol::InRoomResponse) {
        match code {
            caro_protocol::InRoomResponse::YourRoomIsFull(rid) => {
                let conn_state = self.global_state.read().await.get_connection_state();
                match conn_state {
                    caro_protocol::ConnectState::Connected => {
                        self.global_state.write().await.set_player_state(caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected));
                        self.global_state.write().await.set_current_rid(rid);
                        self.screen_manager.write().await.clean();
                        self.screen_manager.write().await.update().await;
                        self.screen_manager.write().await.log("game is ready!".to_string()).await;
                        self.screen_manager.write().await.enable_prompt_mode().await;
                    },
                    caro_protocol::ConnectState::Disconnected => {

                    },
                }
            },
        }
    }

    async fn execute_ingame_response(&mut self, code: caro_protocol::InGameResponse) {
        match code {
            caro_protocol::InGameResponse::MoveSuccess => {

            },
            caro_protocol::InGameResponse::MoveUnsuccess => {

            },
            caro_protocol::InGameResponse::Context(game_context) => {
                // screen_manager::print_caro_context(game_context);
                self.screen_manager.write().await.update_game_context(&game_context);
                self.screen_manager.write().await.update_board_only().await;
            },
        }
    }
}