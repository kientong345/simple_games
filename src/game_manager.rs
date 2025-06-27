use std::{cell::RefCell, rc::Rc, sync::Arc};
use tokio::sync::Mutex;

pub mod simple_caro;

use crate::{
    communication::{self, ToMessagePacket},
    player_manager,
    room_manager,
};

pub enum OperationResult {
    Successfully(simple_caro::GameState),
    RoomNotExist,
    RoomNotFullYet,
    Player1Left,
    Player2Left,
}

#[derive(Debug, Clone)]
struct GameContext {
    board: Rc<RefCell<Vec<Vec<simple_caro::TileState>>>>,
    player1_move_history: Vec<simple_caro::Coordinate>,
    player2_move_history: Vec<simple_caro::Coordinate>,
    player1_undone_moves: Vec<simple_caro::Coordinate>,
    player2_undone_moves: Vec<simple_caro::Coordinate>,
    game_state: simple_caro::GameState,
    player1_state: player_manager::PlayerState,
    player2_state: player_manager::PlayerState,
}

impl communication::ToMessagePacket for GameContext {
    fn to_message_packet(self) -> communication::MessagePacket {
        todo!()
    }
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

        // override player's callbacks
        // let prev_callback1 = self.player_manager.lock().await.get_action_on_request(self.player1_id);
        // let prev_callback2 = self.player_manager.lock().await.get_action_on_request(self.player2_id);
        // self.player_manager.lock().await.set_action_on_request(self.player1_id, |msg| {});
        // self.player_manager.lock().await.set_action_on_request(self.player2_id, |msg| {});

        // set rule
        match self.room_manager.lock().await.get_rule_in_room(rid).unwrap() {
            communication::GameRule::TicTacToe => {
                self.game.set_rule(simple_caro::RuleType::TicTacToe);
                self.game.set_board_size(3, 3);
            }
            communication::GameRule::FourBlockOne => {
                self.game.set_rule(simple_caro::RuleType::FourBlockOne);
                self.game.set_board_size(1024, 1024);
            }
            communication::GameRule::FiveBlockTwo => {
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


        // logic goes here


        let game_result = self.game.get_state();

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

        // return player's callbacks
        // self.player_manager.lock().await.set_action_on_request(self.player1_id, prev_callback1.unwrap());
        // self.player_manager.lock().await.set_action_on_request(self.player2_id, prev_callback2.unwrap());

        OperationResult::Successfully(game_result)
    }

    pub fn get_state(&self) -> simple_caro::GameState {
        self.game.get_state()
    }

    pub fn is_over(&self) -> bool {
        self.game.is_over()
    }

    pub async fn execute_player_command(&mut self, cmd_code: communication::PlayerCommand) {
        match cmd_code {
            communication::PlayerCommand::Player1Move(x, y) => {
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
            communication::PlayerCommand::Player2Move(x, y) => {
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
            communication::PlayerCommand::Player1Undo => {
                let result = self.game.player_undo(simple_caro::Participant::Player1);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                    }
                    _ => {
                    }
                }
            }
            communication::PlayerCommand::Player2Undo => {
                let result = self.game.player_undo(simple_caro::Participant::Player2);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                    }
                    _ => {
                    }
                }
            }
            communication::PlayerCommand::Player1Redo => {
                let result = self.game.player_redo(simple_caro::Participant::Player1);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                    }
                    _ => {
                    }
                }
            }
            communication::PlayerCommand::Player2Redo => {
                let result = self.game.player_redo(simple_caro::Participant::Player2);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.response_context().await;
                    }
                    _ => {
                    }
                }
            }
            communication::PlayerCommand::Player1RequestContext => {
                self.response_context().await;
            }
            communication::PlayerCommand::Player2RequestContext => {
                self.response_context().await;
            }
            communication::PlayerCommand::Player1Leave => {
                self.game.stop();
                self.response_context().await;
            }
            communication::PlayerCommand::Player2Leave => {
                self.game.stop();
                self.response_context().await;
            }
            _ => {
            }
        }
    }

    async fn response_context(&mut self) {
        let game_context = GameContext {
            board: self.game.get_board(),
            player1_move_history: self.game.get_moves_history(simple_caro::Participant::Player1),
            player2_move_history: self.game.get_moves_history(simple_caro::Participant::Player2),
            player1_undone_moves: self.game.get_undone_moves(simple_caro::Participant::Player1),
            player2_undone_moves: self.game.get_undone_moves(simple_caro::Participant::Player2),
            game_state: self.game.get_state(),
            player1_state: self.player_manager.lock().await.get_player_state(self.player1_id).unwrap(),
            player2_state: self.player_manager.lock().await.get_player_state(self.player2_id).unwrap(),
        };
        let game_context_clone = game_context.clone();
        self.player_manager.lock().await.response(self.player1_id, game_context.to_message_packet()).await;
        self.player_manager.lock().await.response(self.player2_id, game_context_clone.to_message_packet()).await;
    }
}