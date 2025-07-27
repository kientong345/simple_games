
use std::sync::Arc;

use tokio::sync::RwLock;

use crate::
{
    caro_protocol,
    client_endpoint,
    global_state,
    input_from_user,
    output_to_user
};

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
                if current_state == caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected) {
                    self.execute_logged_command(command).await;
                }
            },
            input_from_user::UserCommand::InRoom(command) => {
                if current_state == caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected) {
                    self.execute_inroom_command(command).await;
                }
            },
            input_from_user::UserCommand::InGame(command) => {
                if current_state == caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected) {
                    self.execute_ingame_command(command).await;
                }
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
                        let code = caro_protocol::PlayerCode::Logged(caro_protocol::LoggedRequest::RequestRoomAsPlayer1(caro_protocol::GameRule::TicTacToe));
                        let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                        // println!("send: {:?}", new_packet);
                        self.requester.write().await.send_request(new_packet).await;
                    },
                    caro_protocol::GameRule::FourBlockOne => {
                        let code = caro_protocol::PlayerCode::Logged(caro_protocol::LoggedRequest::RequestRoomAsPlayer1(caro_protocol::GameRule::FourBlockOne));
                        let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                        // println!("send: {:?}", new_packet);
                        self.requester.write().await.send_request(new_packet).await;
                    },
                    caro_protocol::GameRule::FiveBlockTwo => {
                        let code = caro_protocol::PlayerCode::Logged(caro_protocol::LoggedRequest::RequestRoomAsPlayer1(caro_protocol::GameRule::FiveBlockTwo));
                        let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                        // println!("send: {:?}", new_packet);
                        self.requester.write().await.send_request(new_packet).await;
                    },
                }
            },
            input_from_user::LoggedCommand::JoinRoom(rid) => {
                let code = caro_protocol::PlayerCode::Logged(caro_protocol::LoggedRequest::JoinRoom(rid));
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
        let cursor_position = self.screen_manager.read().await.get_cursor_pos();
        match command {
            input_from_user::InGameCommand::Move(coor) => {
                let coor = (coor.0, coor.1);
                let code = caro_protocol::PlayerCode::InGame(caro_protocol::InGameRequest::PlayerMove(coor));
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                // println!("send: {:?}", new_packet);
                self.requester.write().await.send_request(new_packet).await;
            },
            input_from_user::InGameCommand::Up => {
                let new_position = (cursor_position.0 - 1, cursor_position.1);
                self.screen_manager.write().await.set_cursor_pos(new_position.0, new_position.1);
                self.screen_manager.write().await.update_board_only().await;
            },
            input_from_user::InGameCommand::Down => {
                let new_position = (cursor_position.0 + 1, cursor_position.1);
                self.screen_manager.write().await.set_cursor_pos(new_position.0, new_position.1);
                self.screen_manager.write().await.update_board_only().await;
            },
            input_from_user::InGameCommand::Left => {
                let new_position = (cursor_position.0, cursor_position.1 - 1);
                self.screen_manager.write().await.set_cursor_pos(new_position.0, new_position.1);
                self.screen_manager.write().await.update_board_only().await;
            },
            input_from_user::InGameCommand::Right => {
                let new_position = (cursor_position.0, cursor_position.1 + 1);
                self.screen_manager.write().await.set_cursor_pos(new_position.0, new_position.1);
                self.screen_manager.write().await.update_board_only().await;
            },
            input_from_user::InGameCommand::Enter => {
                let code = caro_protocol::PlayerCode::InGame(caro_protocol::InGameRequest::PlayerMove(cursor_position));
                let new_packet = caro_protocol::MessagePacket::new_player_packet(code);
                // println!("send: {:?}", new_packet);
                self.requester.write().await.send_request(new_packet).await;
            },
            input_from_user::InGameCommand::Redo => {

            }
            input_from_user::InGameCommand::Undo => {

            },
            input_from_user::InGameCommand::SwitchInputMode => {
                let is_prompt_mode = self.screen_manager.read().await.is_prompt_mode();
                if is_prompt_mode {
                    self.screen_manager.write().await.disable_prompt_mode();
                    println!("disable prompt mode");
                } else {
                    self.screen_manager.write().await.enable_prompt_mode().await;
                    println!("enable prompt mode");
                }
            },
            input_from_user::InGameCommand::LeaveRoom => {

            }
        }
    }
}