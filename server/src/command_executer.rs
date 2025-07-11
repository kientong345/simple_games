use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{caro_protocol, game_manager, player_manager::{self, PlayerManager}, room_manager};

pub struct CommandExecuter {
    player_manager: Arc<Mutex<player_manager::PlayerContainer>>,
    room_manager: Arc<Mutex<room_manager::RoomContainer>>,
    game_manager: Arc<Mutex<game_manager::GameContainer>>,
}

impl CommandExecuter {
    pub fn new(player_manager: Arc<Mutex<player_manager::PlayerContainer>>,
                room_manager: Arc<Mutex<room_manager::RoomContainer>>,
                game_manager: Arc<Mutex<game_manager::GameContainer>>) -> Self {
        Self {
            player_manager,
            room_manager,
            game_manager,
        }
    }

    pub async fn execute_request(&mut self, pid: i32, code: caro_protocol::PlayerCode) {
        let player_state = self.player_manager.lock().await.get_player_state(pid).unwrap();
        match player_state {
            player_manager::PlayerState::Logged(player_manager::ConnectState::Connected) => {
                self.execute_logged_request(pid, code).await;
            },
            player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected) => {
                self.execute_waiting_request(pid, code).await;
            },
            player_manager::PlayerState::InGame(player_manager::ConnectState::Connected) => {
                self.execute_ingame_request(pid, code).await;
            },
            _ => {
                // do nothing
            }
        }
    }

    async fn execute_logged_request(&mut self, pid: i32, code: caro_protocol::PlayerCode) {
        let room_manager_clone = self.room_manager.clone();
        let player_manager_clone = self.player_manager.clone();
        let game_manager_clone = self.game_manager.clone();
        let room_full_actions = |rid: i32| async move {
            let gid = game_manager_clone.lock().await.find_game_contain_room(rid).unwrap();
            game_manager_clone.lock().await.try_start_game(gid);
            let (pid1, pid2) = room_manager_clone.lock().await.get_pids_in_room(rid).unwrap();
            let code = caro_protocol::ServerCode::YourRoomIsFull(rid);
            let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            player_manager_clone.lock().await.response(pid1, new_packet.clone()).await;
            tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            player_manager_clone.lock().await.response(pid2, new_packet).await;
            player_manager_clone.lock().await.set_player_state(pid1, player_manager::PlayerState::InGame(player_manager::ConnectState::Connected));
            player_manager_clone.lock().await.set_player_state(pid2, player_manager::PlayerState::InGame(player_manager::ConnectState::Connected));
        };

        match code {
            caro_protocol::PlayerCode::RequestRoomAsPlayer1(rule_type) => {
                let new_rid = self.room_manager.lock().await.add_room(rule_type);
                if new_rid == -1 {
                    let code = caro_protocol::ServerCode::FailedToCreateRoom;
                    let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                    self.player_manager.lock().await.response(pid, new_packet).await;
                    return;
                }
                let _result = self.room_manager.lock().await.add_player_to_room(new_rid, room_manager::PlayerOrder::Player1(pid));
                if !_result {
                    let code = caro_protocol::ServerCode::FailedToJoinRoom(new_rid);
                    let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                    self.player_manager.lock().await.response(pid, new_packet).await;
                } else {
                    let _new_gid = self.game_manager.lock().await.add_game(new_rid, rule_type);
                    self.player_manager.lock().await.set_player_state(pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                    let code = caro_protocol::ServerCode::JoinedRoomAsPlayer1(new_rid);
                    let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                    self.player_manager.lock().await.response(pid, new_packet).await;
                    if self.room_manager.lock().await.room_full(new_rid) {
                        room_full_actions(new_rid).await;
                    }
                }
            },
            caro_protocol::PlayerCode::JoinRoomAsPlayer2(rid) => {
                let _result = self.room_manager.lock().await.add_player_to_room(rid, room_manager::PlayerOrder::Player2(pid));
                self.player_manager.lock().await.set_player_state(pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                let code = caro_protocol::ServerCode::JoinedRoomAsPlayer2(rid);
                let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                self.player_manager.lock().await.response(pid, new_packet).await;
                if self.room_manager.lock().await.room_full(rid) {
                    room_full_actions(rid).await;
                }
            },
            _ => {
                // do not process other requests
            }
        }
    }  

    async fn execute_waiting_request(&mut self, pid: i32, code: caro_protocol::PlayerCode) {

    }

    async fn execute_ingame_request(&mut self, pid: i32, code: caro_protocol::PlayerCode) {
        let rid = self.room_manager.lock().await.find_room_contain_player(pid).unwrap();
        let gid = self.game_manager.lock().await.find_game_contain_room(rid).unwrap();
        
        let _result = self.game_manager.lock().await.execute_command_in_game(gid, code);

        let internal_game_context = self.game_manager.lock().await.get_context_in_game(gid).unwrap();

        let (pid1, pid2) = self.room_manager.lock().await.get_pids_in_room(rid).unwrap();
        let player1_state = match self.player_manager.lock().await.get_player_state(pid1).unwrap() {
            player_manager::PlayerState::Logged(conn_state) => {
                match conn_state {
                    player_manager::ConnectState::Connected => caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected),
                    player_manager::ConnectState::Disconnected => caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Disconnected),
                }
            },
            player_manager::PlayerState::InGame(conn_state) => {
                match conn_state {
                    player_manager::ConnectState::Connected => caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected),
                    player_manager::ConnectState::Disconnected => caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Disconnected),
                }
            }
            player_manager::PlayerState::Waiting(conn_state) => {
                match conn_state {
                    player_manager::ConnectState::Connected => caro_protocol::PlayerState::Waiting(caro_protocol::ConnectState::Connected),
                    player_manager::ConnectState::Disconnected => caro_protocol::PlayerState::Waiting(caro_protocol::ConnectState::Disconnected),
                }
            }
        };
        
        let player2_state = match self.player_manager.lock().await.get_player_state(pid2).unwrap() {
            player_manager::PlayerState::Logged(conn_state) => {
                match conn_state {
                    player_manager::ConnectState::Connected => caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected),
                    player_manager::ConnectState::Disconnected => caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Disconnected),
                }
            },
            player_manager::PlayerState::InGame(conn_state) => {
                match conn_state {
                    player_manager::ConnectState::Connected => caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected),
                    player_manager::ConnectState::Disconnected => caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Disconnected),
                }
            }
            player_manager::PlayerState::Waiting(conn_state) => {
                match conn_state {
                    player_manager::ConnectState::Connected => caro_protocol::PlayerState::Waiting(caro_protocol::ConnectState::Connected),
                    player_manager::ConnectState::Disconnected => caro_protocol::PlayerState::Waiting(caro_protocol::ConnectState::Disconnected),
                }
            }
        };
        let game_context = caro_protocol::GameContext {
            board: internal_game_context.board,
            player1_move_history: internal_game_context.player1_move_history,
            player2_move_history: internal_game_context.player2_move_history,
            player1_undone_moves: internal_game_context.player1_undone_moves,
            player2_undone_moves: internal_game_context.player2_undone_moves,
            game_state: internal_game_context.game_state,
            player1_state,
            player2_state,
        };

        let new_message_packet = caro_protocol::MessagePacket::new_server_packet(caro_protocol::ServerCode::Context(game_context));

        self.player_manager.lock().await.response(pid1, new_message_packet.clone()).await;
        self.player_manager.lock().await.response(pid2, new_message_packet).await;
    }

}