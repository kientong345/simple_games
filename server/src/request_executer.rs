use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    caro_protocol,
    game_manager,
    player_manager,
    room_manager
};

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

        // global request (regardless of owner's state)
        match code {
            caro_protocol::PlayerCode::PlayerExitApplication => {
                self.clean_player_existence(pid).await;
            },
            caro_protocol::PlayerCode::PlayerRequestState => {
                let player_state = self.player_manager.lock().await.get_player_state(pid).unwrap();
                let code = match player_state {
                    player_manager::PlayerState::Logged(conn_state) => {
                        match conn_state {
                            player_manager::ConnectState::Connected => {
                                caro_protocol::ServerCode::State(caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected))
                            },
                            player_manager::ConnectState::Disconnected => {
                                caro_protocol::ServerCode::State(caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Disconnected))
                            },
                        }
                    },
                    player_manager::PlayerState::Waiting(conn_state) => {
                        match conn_state {
                            player_manager::ConnectState::Connected => {
                                caro_protocol::ServerCode::State(caro_protocol::PlayerState::Waiting(caro_protocol::ConnectState::Connected))
                            },
                            player_manager::ConnectState::Disconnected => {
                                caro_protocol::ServerCode::State(caro_protocol::PlayerState::Waiting(caro_protocol::ConnectState::Disconnected))
                            },
                        }
                    },
                    player_manager::PlayerState::InGame(conn_state) => {
                        match conn_state {
                            player_manager::ConnectState::Connected => {
                                caro_protocol::ServerCode::State(caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected))
                            },
                            player_manager::ConnectState::Disconnected => {
                                caro_protocol::ServerCode::State(caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Disconnected))
                            },
                        }
                    },
                };
                let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                self.player_manager.lock().await.response(pid, new_packet).await;
            }
            _ => {

            }
        }

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

            // let code = caro_protocol::ServerCode::State(caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected));
            // let new_message_packet = caro_protocol::MessagePacket::new_server_packet(code);
            // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            // player_manager_clone.lock().await.response(pid1, new_message_packet.clone()).await;
            // tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            // player_manager_clone.lock().await.response(pid2, new_message_packet).await;
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
            caro_protocol::PlayerCode::JoinRoom(rid) => {
                let _result = self.room_manager.lock().await.add_player_to_room(rid, room_manager::PlayerOrder::Player2(pid));
                self.player_manager.lock().await.set_player_state(pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                let (pid1, pid2) = self.room_manager.lock().await.get_pids_in_room(rid).unwrap();
                let code = if pid == pid1 {
                    caro_protocol::ServerCode::JoinedRoomAsPlayer1(rid)
                } else if pid == pid2 {
                    caro_protocol::ServerCode::JoinedRoomAsPlayer2(rid)
                } else {
                    caro_protocol::ServerCode::FailedToJoinRoom(rid)
                };
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
        match code {
            caro_protocol::PlayerCode::PlayerLeaveRoom => {
                let rid = self.room_manager.lock().await.find_room_contain_player(pid).unwrap();
                self.room_manager.lock().await.remove_player_from_room(rid, pid);
                let room_empty = self.room_manager.lock().await.room_empty(rid);
                if room_empty {
                    self.room_manager.lock().await.remove_room(rid);
                }
            },
            _ => {

            }
        }
    }

    async fn execute_ingame_request(&mut self, pid: i32, code: caro_protocol::PlayerCode) {
        let rid = self.room_manager.lock().await.find_room_contain_player(pid).unwrap();
        let gid = self.game_manager.lock().await.find_game_contain_room(rid).unwrap();

        match code {
            caro_protocol::PlayerCode::PlayerLeaveRoom => {
                self.room_manager.lock().await.remove_player_from_room(rid, pid);
                let room_empty = self.room_manager.lock().await.room_empty(rid);
                if room_empty {
                    self.game_manager.lock().await.remove_game(gid);
                    self.room_manager.lock().await.remove_room(rid);
                }
            },
            caro_protocol::PlayerCode::PlayerRequestContext => {
                self.response_game_context(pid).await;
            },
            _ => {

            }
        }

        let (pid1, pid2) = self.room_manager.lock().await.get_pids_in_room(rid).unwrap();
        let player_order = if pid == pid1 {
            game_manager::PlayerOrder::Player1
        } else if pid == pid2 {
            game_manager::PlayerOrder::Player2
        } else {
            return;
        };
        
        let result = self.game_manager.lock().await.execute_command_in_game(gid, player_order, code).unwrap();

        match result {
            game_manager::OperationResult::Successfully(_game_state) => {

            },
            game_manager::OperationResult::Unsuccessfully(_game_state) => {
                
            },
        }

        self.response_game_context(pid1).await;
        self.response_game_context(pid2).await;
    }

    async fn response_game_context(&self, pid: i32) {
        let rid = self.room_manager.lock().await.find_room_contain_player(pid).unwrap();
        let gid = self.game_manager.lock().await.find_game_contain_room(rid).unwrap();
        let (pid1, pid2) = self.room_manager.lock().await.get_pids_in_room(rid).unwrap();
        let internal_game_context = self.game_manager.lock().await.get_context_in_game(gid).unwrap();

        let player1_state = self.player_manager.lock().await.get_player_state(pid1).unwrap();
        let mut player1_connection_state = caro_protocol::ConnectState::Disconnected;
        if let player_manager::PlayerState::InGame(conn_state) = player1_state {
            match conn_state {
                player_manager::ConnectState::Connected => player1_connection_state = caro_protocol::ConnectState::Connected,
                player_manager::ConnectState::Disconnected => player1_connection_state = caro_protocol::ConnectState::Disconnected,
            }
        };

        let player2_state = self.player_manager.lock().await.get_player_state(pid2).unwrap();
        let mut player2_connection_state = caro_protocol::ConnectState::Disconnected;
        if let player_manager::PlayerState::InGame(conn_state) = player2_state {
            match conn_state {
                player_manager::ConnectState::Connected => player2_connection_state = caro_protocol::ConnectState::Connected,
                player_manager::ConnectState::Disconnected => player2_connection_state = caro_protocol::ConnectState::Disconnected,
            }
        };

        let receiver_order = if pid == pid1 {
            caro_protocol::PlayerOrder::Player1
        } else if pid == pid2 {
            caro_protocol::PlayerOrder::Player2
        } else {
            return;
        };

        let game_context = caro_protocol::GameContext {
            board_height: internal_game_context.board_height,
            board_width: internal_game_context.board_width,
            player1_move_history: internal_game_context.player1_move_history,
            player2_move_history: internal_game_context.player2_move_history,
            player1_undone_moves: internal_game_context.player1_undone_moves,
            player2_undone_moves: internal_game_context.player2_undone_moves,
            game_state: internal_game_context.game_state,
            player1_connection_state,
            player2_connection_state,
            receiver_order,
        };

        let new_message_packet = caro_protocol::MessagePacket::new_server_packet(caro_protocol::ServerCode::Context(game_context.clone()));
        self.player_manager.lock().await.response(pid, new_message_packet).await;
    }

    async fn clean_player_existence(&mut self, pid: i32) {
        let rid = self.room_manager.lock().await.find_room_contain_player(pid).unwrap();
        let gid = self.game_manager.lock().await.find_game_contain_room(rid).unwrap();
        self.room_manager.lock().await.remove_player_from_room(rid, pid);
        let room_empty = self.room_manager.lock().await.room_empty(rid);
        if room_empty {
            self.game_manager.lock().await.remove_game(gid);
            self.room_manager.lock().await.remove_room(rid);
        }
        self.player_manager.lock().await.remove_player(pid);
    }

}