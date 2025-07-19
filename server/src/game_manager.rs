use std::collections::HashMap;
use simple_caro;

use crate::id_pool;
use crate::caro_protocol;

pub enum OperationResult {
    Successfully(simple_caro::GameState),
    Unsuccessfully(simple_caro::GameState),
}

#[derive(Debug, Clone, Copy)]
enum GameAvailability {
    Pending,
    Started,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerOrder {
    Player1,
    Player2,
}

pub struct InternalGameContext {
    pub board_height: usize,
    pub board_width: usize,
    pub player1_move_history: Vec<caro_protocol::Coordinate>,
    pub player2_move_history: Vec<caro_protocol::Coordinate>,
    pub player1_undone_moves: Vec<caro_protocol::Coordinate>,
    pub player2_undone_moves: Vec<caro_protocol::Coordinate>,
    pub game_state: caro_protocol::GameState,
}

pub struct GameOperator {
    game: simple_caro::SimpleCaro,
    room_id: i32,
}

impl GameOperator {
    fn new(room_id: i32, game_rule: caro_protocol::GameRule) -> Self {
        let game = simple_caro::SimpleCaro::new();
        match game_rule {
            caro_protocol::GameRule::TicTacToe => {
                game.set_rule(simple_caro::RuleType::TicTacToe);
                game.set_board_size(3, 3);
            }
            caro_protocol::GameRule::FourBlockOne => {
                game.set_rule(simple_caro::RuleType::FourBlockOne);
                game.set_board_size(1024, 1024);
            }
            caro_protocol::GameRule::FiveBlockTwo => {
                game.set_rule(simple_caro::RuleType::FiveBlockTwo);
                game.set_board_size(1024, 1024);
            }
        }
        Self {
            game,
            room_id,
        }
    }

    fn try_start(&mut self) -> bool {
        match self.get_availability() {
            GameAvailability::Pending => {
                self.game.start(simple_caro::GameState::Player1Turn);
                true
            }
            GameAvailability::Started => {
                false
            }
        }
    }

    fn try_stop(&mut self) -> bool {
        match self.get_availability() {
            GameAvailability::Pending => {
                self.game.stop();
                true
            }
            GameAvailability::Started => {
                false
            }
        }
    }

    fn get_availability(&self) -> GameAvailability {
        match self.get_state() {
            caro_protocol::GameState::Player1Turn => GameAvailability::Started,
            caro_protocol::GameState::Player2Turn => GameAvailability::Started,
            caro_protocol::GameState::Player1Won => GameAvailability::Pending,
            caro_protocol::GameState::Player2Won => GameAvailability::Pending,
            caro_protocol::GameState::Drew => GameAvailability::Pending,
            caro_protocol::GameState::NotInprogress => GameAvailability::Pending,
        }
    }

    fn get_board_height(&self) -> usize {
        self.game.get_board_height()
    }

    fn get_board_width(&self) -> usize {
        self.game.get_board_width()
    }

    fn get_board(&self) -> Vec<caro_protocol::Row> {
        let mut board = Vec::<Vec<caro_protocol::TileState>>::new();
        for latitude in 0..self.game.get_board_height() {
            let mut row = Vec::<caro_protocol::TileState>::new();
            for longtitude in 0..self.game.get_board_width() {
                match self.game.get_board_tile(latitude, longtitude) {
                    simple_caro::TileState::Player1 => row.push(caro_protocol::TileState::Player1),
                    simple_caro::TileState::Player2 => row.push(caro_protocol::TileState::Player2),
                    simple_caro::TileState::Empty => row.push(caro_protocol::TileState::Empty),
                }
            }
            board.push(row);
        }
        board
    }

    fn get_player_move_history(&self, order: PlayerOrder) -> Vec<caro_protocol::Coordinate> {
        match order {
            PlayerOrder::Player1 => {
                let mut player1_move_history = Vec::<caro_protocol::Coordinate>::new();
                for move_lib in self.game.get_moves_history(simple_caro::Participant::Player1) {
                    player1_move_history.push((move_lib.latitude, move_lib.longtitude));
                }
                player1_move_history
            },
            PlayerOrder::Player2 => {
                let mut player2_move_history = Vec::<caro_protocol::Coordinate>::new();
                for move_lib in self.game.get_moves_history(simple_caro::Participant::Player2) {
                    player2_move_history.push((move_lib.latitude, move_lib.longtitude));
                }
                player2_move_history
            }
        }
    }

    fn get_player_undone_moves(&self, order: PlayerOrder) -> Vec<caro_protocol::Coordinate> {
        match order {
            PlayerOrder::Player1 => {
                let mut player1_undone_moves = Vec::<caro_protocol::Coordinate>::new();
                for move_lib in self.game.get_undone_moves(simple_caro::Participant::Player1) {
                    player1_undone_moves.push((move_lib.latitude, move_lib.longtitude));
                }
                player1_undone_moves
            },
            PlayerOrder::Player2 => {
                let mut player2_undone_moves = Vec::<caro_protocol::Coordinate>::new();
                for move_lib in self.game.get_undone_moves(simple_caro::Participant::Player2) {
                    player2_undone_moves.push((move_lib.latitude, move_lib.longtitude));
                }
                player2_undone_moves
            }
        }
    }

    fn get_state(&self) -> caro_protocol::GameState {
        match self.game.get_state() {
            simple_caro::GameState::Player1Turn => caro_protocol::GameState::Player1Turn,
            simple_caro::GameState::Player2Turn => caro_protocol::GameState::Player2Turn,
            simple_caro::GameState::Player1Won => caro_protocol::GameState::Player1Won,
            simple_caro::GameState::Player2Won => caro_protocol::GameState::Player2Won,
            simple_caro::GameState::Drew => caro_protocol::GameState::Drew,
            simple_caro::GameState::NotInprogress => caro_protocol::GameState::NotInprogress,
        }
    }

    fn execute_command(&mut self, player_order: PlayerOrder, cmd_code: caro_protocol::PlayerCode) -> OperationResult {
        let mut is_success = false;
        let who = match player_order {
            PlayerOrder::Player1 => simple_caro::Participant::Player1,
            PlayerOrder::Player2 => simple_caro::Participant::Player2,
        };
        match cmd_code {
            caro_protocol::PlayerCode::PlayerMove((latitude, longtitude)) => {
                let pos = simple_caro::Coordinate {latitude, longtitude};
                let result = self.game.player_move(who, pos);
                match result {
                    simple_caro::MoveResult::Success => {
                        self.game.switch_turn();
                        is_success = true;
                    }
                    _ => {
                        is_success = false;
                    }
                }
            }
            caro_protocol::PlayerCode::PlayerUndo => {
                let result = self.game.player_undo(who);
                match result {
                    simple_caro::MoveResult::Success => {
                        is_success = true;
                    }
                    _ => {
                        is_success = false;
                    }
                }
            }
            caro_protocol::PlayerCode::PlayerRedo => {
                let result = self.game.player_redo(who);
                match result {
                    simple_caro::MoveResult::Success => {
                        is_success = true;
                    }
                    _ => {
                        is_success = false;
                    }
                }
            }
            _ => {
                // do not process other requests
            }
        }
        
        if is_success {
            OperationResult::Successfully(self.game.get_state())
        } else {
            OperationResult::Unsuccessfully(self.game.get_state())
        }
    }

    fn get_rid(&self) -> i32 {
        self.room_id
    }
}

pub struct GameContainer {
    games_set: HashMap<i32, GameOperator>,
    max_games: usize,
    gid_pool: id_pool::IdPool,
}

impl GameContainer {
    pub fn new(max_games: usize, gid_pool: id_pool::IdPool) -> Self {
        Self {
            games_set: HashMap::<i32, GameOperator>::new(),
            max_games,
            gid_pool,
        }
    }

    pub fn add_game(&mut self, rid: i32, game_rule: caro_protocol::GameRule) -> i32 {
        if self.games_set.len() >= self.max_games {
            return -1;
        }
        let new_gid = self.gid_pool.alloc_id();
        let new_game = GameOperator::new(rid, game_rule);
        self.games_set.insert(new_gid, new_game);
        new_gid
    }

    pub fn remove_game(&mut self, gid: i32) {
        self.gid_pool.dealloc_id(gid);
        self.games_set.remove(&gid);
    }

    pub fn try_start_game(&mut self, gid: i32) -> bool {
        if let Some(game) = self.games_set.get_mut(&gid) {
            game.try_start()
        } else {
            false
        }
    }

    pub fn try_stop_game(&mut self, gid: i32) -> bool {
        if let Some(game) = self.games_set.get_mut(&gid) {
            game.try_stop()
        } else {
            false
        }
    }

    pub fn get_context_in_game(&self, gid: i32) -> Option<InternalGameContext> {
        if let Some(game) = self.games_set.get(&gid) {
            Some(InternalGameContext {
                board_height: game.get_board_height(),
                board_width: game.get_board_width(),
                player1_move_history: game.get_player_move_history(PlayerOrder::Player1),
                player2_move_history: game.get_player_move_history(PlayerOrder::Player2),
                player1_undone_moves: game.get_player_undone_moves(PlayerOrder::Player1),
                player2_undone_moves: game.get_player_undone_moves(PlayerOrder::Player2),
                game_state: game.get_state(),
            })
        } else {
            None
        }
    }

    pub fn execute_command_in_game(&mut self, gid: i32, player_order: PlayerOrder, cmd_code: caro_protocol::PlayerCode) -> Option<OperationResult> {
        if let Some(game) = self.games_set.get_mut(&gid) {
            Some(game.execute_command(player_order, cmd_code))
        } else {
            None
        }
    }

    pub fn find_game_contain_room(&self, rid: i32) -> Option<i32> {
        let target = self.games_set.iter().find(|&(_gid, game)| {
            let its_rid = game.get_rid();
            its_rid == rid
        });
        if let Some((gid, _game)) = target {
            Some(*gid)
        } else {
            None
        }
    }
}
