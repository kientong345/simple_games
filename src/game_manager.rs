use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

pub mod simple_caro;

use crate::{
    caro_protocol, make_action, player_manager, room_manager
};

pub enum OperationResult {
    Successfully(simple_caro::GameState),
    RoomNotExist,
    RoomNotFullYet,
    Player1Left,
    Player2Left,
}

pub struct GameOperator<A: player_manager::PlayerManager, B: room_manager::RoomManager> {
    game: simple_caro::SimpleCaro,
    player1_id: i32,
    player2_id: i32,
    player_manager: Arc<Mutex<A>>,
    room_manager: Arc<Mutex<B>>,
}

impl<A, B> GameOperator<A, B>
where A: player_manager::PlayerManager, B: room_manager::RoomManager {
    pub fn new(player_manager: Arc<Mutex<A>>, room_manager: Arc<Mutex<B>>) -> Self {
        let game = simple_caro::SimpleCaro::new();
        Self {
            game,
            player1_id: -1,
            player2_id: -1,
            player_manager,
            room_manager,
        }
    }

    pub async fn try_operate_in_room(&mut self, rid: i32) -> OperationResult {
        // this function should run asynchronously btw

        if !self.room_manager.lock().await.room_exist(rid) {
            return OperationResult::RoomNotExist;
        }

        if !self.room_manager.lock().await.room_full(rid) {
            return OperationResult::RoomNotFullYet;
        }

        // set rule
        match self.room_manager.lock().await.get_rule_in_room(rid).unwrap() {
            caro_protocol::GameRule::TicTacToe => {
                self.game.set_rule(simple_caro::RuleType::TicTacToe);
                self.game.set_board_size(3, 3);
            }
            caro_protocol::GameRule::FourBlockOne => {
                self.game.set_rule(simple_caro::RuleType::FourBlockOne);
                self.game.set_board_size(1024, 1024);
            }
            caro_protocol::GameRule::FiveBlockTwo => {
                self.game.set_rule(simple_caro::RuleType::FiveBlockTwo);
                self.game.set_board_size(1024, 1024);
            }
        }
        
        let (player1_id, player2_id) = self.room_manager.lock().await.get_pids_in_room(rid).unwrap();

        // set player's states
        self.player1_id = player1_id;
        let player1_connection_state = match self.player_manager.lock().await.get_player_state(self.player1_id).unwrap() {
            player_manager::PlayerState::Logged(state) => state,
            player_manager::PlayerState::Waiting(state) => state,
            player_manager::PlayerState::InGame(state) => state,
        };
        self.player_manager.lock().await.set_player_state(self.player1_id, player_manager::PlayerState::InGame(player1_connection_state));
        
        self.player2_id = player2_id;
        let player2_connection_state = match self.player_manager.lock().await.get_player_state(self.player2_id).unwrap() {
            player_manager::PlayerState::Logged(state) => state,
            player_manager::PlayerState::Waiting(state) => state,
            player_manager::PlayerState::InGame(state) => state,
        };
        self.player_manager.lock().await.set_player_state(self.player2_id, player_manager::PlayerState::InGame(player2_connection_state));

        self.game.start(simple_caro::GameState::Player1Turn);

        // override player's callbacks
        let player_manager_clone = self.player_manager.clone();
        let prev_callback1 = player_manager_clone.lock().await.get_action_on_request(self.player1_id).await;
        let prev_callback2 = player_manager_clone.lock().await.get_action_on_request(self.player2_id).await;

        let (tx, mut rx) = mpsc::channel::<caro_protocol::PlayerCode>(2);

        let tx_clone = tx.clone();

        self.player_manager.lock().await.set_action_on_request(
            self.player1_id,
            make_action!(move |msg: caro_protocol::MessagePacket| {
                let tx_clone = tx_clone.clone();
                let future = async move {
                    if let caro_protocol::GenericCode::Player(code) = msg.code() {
                        tx_clone.send(code).await.unwrap();
                    }
                };
                Box::pin(future) as futures::future::BoxFuture<'static, ()>
            })
        ).await;

        self.player_manager.lock().await.set_action_on_request(
            self.player2_id,
            make_action!(move |msg: caro_protocol::MessagePacket| {
                let tx_clone = tx.clone();
                let future = async move {
                    if let caro_protocol::GenericCode::Player(code) = msg.code() {
                        tx_clone.send(code).await.unwrap();
                    }
                };
                Box::pin(future) as futures::future::BoxFuture<'static, ()>
            })
        ).await;

        self.response_context().await;

        loop {
            if let Some(code) = rx.recv().await {
                if !self.execute_player_command(code).await {
                    break;
                }
            }
        }

        let game_result = self.game.get_state();

        self.response_context().await;

        // return player's callbacks
        self.player_manager.lock().await.set_action_on_request(self.player1_id, prev_callback1).await;
        self.player_manager.lock().await.set_action_on_request(self.player2_id, prev_callback2).await;

        self.game.stop();

        // unset player's states
        self.player1_id = -1;
        let player1_connection_state = match self.player_manager.lock().await.get_player_state(self.player1_id).unwrap() {
            player_manager::PlayerState::Logged(state) => state,
            player_manager::PlayerState::Waiting(state) => state,
            player_manager::PlayerState::InGame(state) => state,
        };
        self.player_manager.lock().await.set_player_state(self.player1_id, player_manager::PlayerState::Waiting(player1_connection_state));
        
        self.player2_id = -1;
        let player2_connection_state = match self.player_manager.lock().await.get_player_state(self.player2_id).unwrap() {
            player_manager::PlayerState::Logged(state) => state,
            player_manager::PlayerState::Waiting(state) => state,
            player_manager::PlayerState::InGame(state) => state,
        };
        self.player_manager.lock().await.set_player_state(self.player2_id, player_manager::PlayerState::Waiting(player2_connection_state));

        // unset rule
        self.game.unset_rule();

        OperationResult::Successfully(game_result)
    }

    pub fn get_state(&self) -> simple_caro::GameState {
        self.game.get_state()
    }

    pub fn is_over(&self) -> bool {
        self.game.is_over()
    }

    pub async fn execute_player_command(&mut self, cmd_code: caro_protocol::PlayerCode) -> bool {
        match cmd_code {
            caro_protocol::PlayerCode::Player1Move(x, y) => {
                let pos = simple_caro::Coordinate {x, y};
                let result = self.game.player_move(simple_caro::Participant::Player1, pos);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                        self.game.switch_turn();
                    }
                    _ => {
                    }
                }
            }
            caro_protocol::PlayerCode::Player2Move(x, y) => {
                let pos = simple_caro::Coordinate {x, y};
                let result = self.game.player_move(simple_caro::Participant::Player2, pos);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                        self.game.switch_turn();
                    }
                    _ => {
                    }
                }
            }
            caro_protocol::PlayerCode::Player1Undo => {
                let result = self.game.player_undo(simple_caro::Participant::Player1);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                    }
                    _ => {
                    }
                }
            }
            caro_protocol::PlayerCode::Player2Undo => {
                let result = self.game.player_undo(simple_caro::Participant::Player2);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                    }
                    _ => {
                    }
                }
            }
            caro_protocol::PlayerCode::Player1Redo => {
                let result = self.game.player_redo(simple_caro::Participant::Player1);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                    }
                    _ => {
                    }
                }
            }
            caro_protocol::PlayerCode::Player2Redo => {
                let result = self.game.player_redo(simple_caro::Participant::Player2);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                    }
                    _ => {
                    }
                }
            }
            caro_protocol::PlayerCode::Player1RequestContext => {
                self.response_context().await;
            }
            caro_protocol::PlayerCode::Player2RequestContext => {
                self.response_context().await;
            }
            caro_protocol::PlayerCode::Player1Leave => {
                return false;
            }
            caro_protocol::PlayerCode::Player2Leave => {
                return false;
            }
            caro_protocol::PlayerCode::RequestRoomAsPlayer1(_game_rule) => {
                // do not process this request
            }
            caro_protocol::PlayerCode::JoinRoomAsPlayer2(_rid) => {
                // do not process this request
            }
        }
        !self.game.is_over()
    }

    async fn response_context(&mut self) {
        let mut board = Vec::<Vec<caro_protocol::TileState>>::new();
        for row in self.game.get_board().borrow().iter() {
            let mut board_row = Vec::<caro_protocol::TileState>::new();
            for tile in row {
                match tile {
                    simple_caro::TileState::Empty => board_row.push(caro_protocol::TileState::Empty),
                    simple_caro::TileState::Player1 => board_row.push(caro_protocol::TileState::Player1),
                    simple_caro::TileState::Player2 => board_row.push(caro_protocol::TileState::Player2),
                }
            }
            board.push(board_row);
        }

        let mut player1_move_history = Vec::<caro_protocol::Coordinate>::new();
        for move_lib in self.game.get_moves_history(simple_caro::Participant::Player1) {
            player1_move_history.push(caro_protocol::Coordinate {
                x: move_lib.x,
                y: move_lib.y,
            });
        }

        let mut player2_move_history = Vec::<caro_protocol::Coordinate>::new();
        for move_lib in self.game.get_moves_history(simple_caro::Participant::Player2) {
            player2_move_history.push(caro_protocol::Coordinate {
                x: move_lib.x,
                y: move_lib.y,
            });
        }

        let mut player1_undone_moves = Vec::<caro_protocol::Coordinate>::new();
        for move_lib in self.game.get_undone_moves(simple_caro::Participant::Player1) {
            player1_undone_moves.push(caro_protocol::Coordinate {
                x: move_lib.x,
                y: move_lib.y,
            });
        }

        let mut player2_undone_moves = Vec::<caro_protocol::Coordinate>::new();
        for move_lib in self.game.get_undone_moves(simple_caro::Participant::Player2) {
            player2_undone_moves.push(caro_protocol::Coordinate {
                x: move_lib.x,
                y: move_lib.y,
            });
        }
        
        let game_state = match self.game.get_state() {
            simple_caro::GameState::Player1Turn => caro_protocol::GameState::Player1Turn,
            simple_caro::GameState::Player2Turn => caro_protocol::GameState::Player2Turn,
            simple_caro::GameState::Player1Won => caro_protocol::GameState::Player1Won,
            simple_caro::GameState::Player2Won => caro_protocol::GameState::Player2Won,
            simple_caro::GameState::Drew => caro_protocol::GameState::Drew,
            simple_caro::GameState::NotInprogress => caro_protocol::GameState::NotInprogress,
        };
        
        let player1_state = match self.player_manager.lock().await.get_player_state(self.player1_id).unwrap() {
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
        
        let player2_state = match self.player_manager.lock().await.get_player_state(self.player2_id).unwrap() {
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
            board,
            player1_move_history,
            player2_move_history,
            player1_undone_moves,
            player2_undone_moves,
            game_state,
            player1_state,
            player2_state,
        };

        let new_message_packet = caro_protocol::MessagePacket::new_server_packet(caro_protocol::ServerCode::Context(game_context));

        self.player_manager.lock().await.response(self.player1_id, new_message_packet.clone()).await;
        self.player_manager.lock().await.response(self.player2_id, new_message_packet).await;
    }
}