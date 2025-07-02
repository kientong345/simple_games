pub use std::{cell::RefCell, rc::Rc};

include!("../../lib/Simple_Caro.rs");

#[derive(Debug, Clone)]
pub struct Coordinate {
    pub latitude: i64,
    pub longtitude: i64,
}

#[derive(Clone, Debug)]
pub enum TileState {
    Empty,
    Player1,
    Player2,
}

#[derive(Debug, Clone)]
pub enum MoveResult {
    Success,
    AlreadyOccupied,
    WrongTurn,
    OutOfBound,
}

#[derive(Debug, Clone)]
pub enum GameState {
    Player1Turn,
    Player2Turn,
    Player1Won,
    Player2Won,
    Drew,
    NotInprogress,
}

#[derive(Debug, Clone)]
pub enum RuleType {
    TicTacToe,
    FourBlockOne,
    FiveBlockTwo,
}

#[derive(Debug, Clone)]
pub enum Participant {
    Player1,
    Player2,
}

pub struct SimpleCaro {
    gid : i32, // for game id
}

impl SimpleCaro {
    pub fn new() -> Self {
        Self {
            gid: unsafe {caro_init_game()},
        }
    }

    pub fn set_board_size(&self, width: i32, height: i32) {
        unsafe {caro_set_board_size(self.gid, width, height);}
    }

    pub fn get_board_width(&self) -> i32 {
        unsafe {caro_get_board_width(self.gid) as i32}
    }

    pub fn get_board_height(&self) -> i32 {
        unsafe {caro_get_board_height(self.gid) as i32}
    }

    pub fn set_rule(&self, rule: RuleType) {
        match rule {
            RuleType::TicTacToe => unsafe {caro_set_rule(self.gid, CARO_RULE_TYPE_CARO_TIC_TAC_TOE);}
            RuleType::FourBlockOne => unsafe {caro_set_rule(self.gid, CARO_RULE_TYPE_CARO_FOUR_BLOCK_1);}
            RuleType::FiveBlockTwo => unsafe {caro_set_rule(self.gid, CARO_RULE_TYPE_CARO_FIVE_BLOCK_2);}
        }
    }

    pub fn unset_rule(&self) {
        unsafe {caro_unset_rule(self.gid);}
    }

    pub fn start(&self, first_turn_state: GameState) {
        match first_turn_state {
            GameState::Player1Turn => unsafe {caro_start(self.gid, CARO_GAME_STATE_CARO_PLAYER1_TURN);}
            GameState::Player2Turn => unsafe {caro_start(self.gid, CARO_GAME_STATE_CARO_PLAYER2_TURN);}
            _ => ()
        }
    }

    pub fn stop(&self) {
        unsafe {caro_stop(self.gid);}
    }

    pub fn player_move(&self, who: Participant, pos: Coordinate) -> MoveResult {
        let c_move = CARO_Coordinate {
            latitude: pos.latitude,
            longtitude: pos.longtitude,
        };
        let result: CARO_MOVE_RESULT;
        match who {
            Participant::Player1 => unsafe {result = caro_player_move(self.gid, CARO_PARTICIPANT_CARO_PLAYER1, c_move);},
            Participant::Player2 => unsafe {result = caro_player_move(self.gid, CARO_PARTICIPANT_CARO_PLAYER2, c_move);},
        }
        match result {
            CARO_MOVE_RESULT_CARO_SUCCESS => MoveResult::Success,
            CARO_MOVE_RESULT_CARO_ALREADY_OCCUPIED => MoveResult::AlreadyOccupied,
            CARO_MOVE_RESULT_CARO_WRONG_TURN => MoveResult::WrongTurn,
            CARO_MOVE_RESULT_CARO_OUT_OF_BOUNDS => MoveResult::OutOfBound,
            _ => MoveResult::OutOfBound,
        }
    }

    pub fn player_undo(&self, who: Participant) -> MoveResult {
        let result: CARO_MOVE_RESULT;
        match who {
            Participant::Player1 => unsafe {result = caro_player_undo(self.gid, CARO_PARTICIPANT_CARO_PLAYER1);},
            Participant::Player2 => unsafe {result = caro_player_undo(self.gid, CARO_PARTICIPANT_CARO_PLAYER2);},
        }
        match result {
            CARO_MOVE_RESULT_CARO_SUCCESS => MoveResult::Success,
            CARO_MOVE_RESULT_CARO_ALREADY_OCCUPIED => MoveResult::AlreadyOccupied,
            CARO_MOVE_RESULT_CARO_WRONG_TURN => MoveResult::WrongTurn,
            CARO_MOVE_RESULT_CARO_OUT_OF_BOUNDS => MoveResult::OutOfBound,
            _ => MoveResult::OutOfBound,
        }
    }

    pub fn player_redo(&self, who: Participant) -> MoveResult {
        let result: CARO_MOVE_RESULT;
        match who {
            Participant::Player1 => unsafe {result = caro_player_redo(self.gid, CARO_PARTICIPANT_CARO_PLAYER1);},
            Participant::Player2 => unsafe {result = caro_player_redo(self.gid, CARO_PARTICIPANT_CARO_PLAYER2);},
        }
        match result {
            CARO_MOVE_RESULT_CARO_SUCCESS => MoveResult::Success,
            CARO_MOVE_RESULT_CARO_ALREADY_OCCUPIED => MoveResult::AlreadyOccupied,
            CARO_MOVE_RESULT_CARO_WRONG_TURN => MoveResult::WrongTurn,
            CARO_MOVE_RESULT_CARO_OUT_OF_BOUNDS => MoveResult::OutOfBound,
            _ => MoveResult::OutOfBound,
        }
    }

    pub fn switch_turn(&self) {
        unsafe {caro_switch_turn(self.gid);}
    }

    // pub fn get_board(&self) -> Rc<RefCell<Vec<Vec<TileState>>>> {
    //     let mut c_board = std::mem::MaybeUninit::<CARO_Board_Struct>::uninit();
    //     unsafe {caro_get_board(self.gid, c_board.as_mut_ptr());}
    //     let board = Rc::new(RefCell::new(Vec::<Vec<TileState>>::new()));
    //     unsafe {
    //         let mut c_board = c_board.assume_init();
    //         board.borrow_mut().resize(c_board.height as usize, Vec::<TileState>::new());
    //         for k in 0..c_board.height {
    //             board.borrow_mut()[k as usize].resize(c_board.width as usize, TileState::Empty);
    //         }
    //         if !c_board.board.is_null() {
    //             for i in 0..c_board.height as usize {
    //                 let row_ptr = *c_board.board.add(i);
    //                 for j in 0..c_board.width as usize {
    //                     let tile = *row_ptr.add(j);
    //                     board.borrow_mut()[i][j] = match tile {
    //                         CARO_TILE_STATE_CARO_TILE_EMPTY => TileState::Empty,
    //                         CARO_TILE_STATE_CARO_TILE_PLAYER1 => TileState::Player1,
    //                         CARO_TILE_STATE_CARO_TILE_PLAYER2 => TileState::Player2,
    //                         _ => TileState::Empty,
    //                     };
    //                 }
    //             }
    //         }
    //         caro_free_board(&mut c_board as *mut CARO_Board_Struct);
    //     }
    //     board
    // }

    pub fn occupied_tiles_count(&self) -> i64 {
        unsafe {caro_occupied_tiles_count(self.gid)}
    }

    pub fn get_board_row(&self, latitude: i32) -> Vec<TileState> {
        todo!()
    }

    pub fn get_board_column(&self, longtitude: i32) -> Vec<TileState> {
        todo!()
    }

    pub fn get_board_tile(&self, latitude: i32, longtitude: i32) -> TileState {
        let tile_state = unsafe {caro_get_tile_state(self.gid, latitude, longtitude)};
        match tile_state {
            CARO_TILE_STATE_CARO_TILE_EMPTY => TileState::Empty,
            CARO_TILE_STATE_CARO_TILE_PLAYER1 => TileState::Player1,
            CARO_TILE_STATE_CARO_TILE_PLAYER2 => TileState::Player2,
            _ => TileState::Empty,
        }
    }

    pub fn get_state(&self) -> GameState {
        let state: CARO_GAME_STATE = unsafe {caro_get_state(self.gid)};
        match state {
            CARO_GAME_STATE_CARO_PLAYER1_TURN => GameState::Player1Turn,
            CARO_GAME_STATE_CARO_PLAYER2_TURN => GameState::Player2Turn,
            CARO_GAME_STATE_CARO_PLAYER1_WON => GameState::Player1Won,
            CARO_GAME_STATE_CARO_PLAYER2_WON => GameState::Player2Won,
            CARO_GAME_STATE_CARO_DREW => GameState::Drew,
            CARO_GAME_STATE_CARO_NOT_INPROGRESS => GameState::NotInprogress,
            _ => GameState::NotInprogress,
        }
    }

    pub fn is_over(&self) -> bool {
        unsafe {caro_is_over(self.gid)}
    }

    pub fn get_moves_history(&self, who: Participant) -> Vec<Coordinate> {
        let mut c_moves_history = std::mem::MaybeUninit::<CARO_Moves_Set>::uninit();
        match who {
            Participant::Player1 => unsafe {caro_get_moves_history(self.gid, c_moves_history.as_mut_ptr(), CARO_PARTICIPANT_CARO_PLAYER1);},
            Participant::Player2 => unsafe {caro_get_moves_history(self.gid, c_moves_history.as_mut_ptr(), CARO_PARTICIPANT_CARO_PLAYER2);},
        }
        let mut moves_history = Vec::new();
        unsafe {
            let mut c_moves_history = c_moves_history.assume_init();
            if !c_moves_history.moves_set.is_null() {
                for i in 0..c_moves_history.length {
                    let pos = Coordinate {
                        latitude: (*c_moves_history.moves_set.add(i)).latitude,
                        longtitude: (*c_moves_history.moves_set.add(i)).longtitude,
                    };
                    moves_history.push(pos);
                }
            }
            caro_free_move_set(&mut c_moves_history as *mut CARO_Moves_Set);
        }
        moves_history
    }

    pub fn get_undone_moves(&self, who: Participant) -> Vec<Coordinate> {
        let mut c_undone_moves = std::mem::MaybeUninit::<CARO_Moves_Set>::uninit();
        match who {
            Participant::Player1 => unsafe {caro_get_undone_moves(self.gid, c_undone_moves.as_mut_ptr(), CARO_PARTICIPANT_CARO_PLAYER1);},
            Participant::Player2 => unsafe {caro_get_undone_moves(self.gid, c_undone_moves.as_mut_ptr(), CARO_PARTICIPANT_CARO_PLAYER2);},
        }
        let mut undone_moves = Vec::new();
        unsafe {
            let mut c_undone_moves = c_undone_moves.assume_init();
            if !c_undone_moves.moves_set.is_null() {
                for i in 0..c_undone_moves.length {
                    let pos = Coordinate {
                        latitude: (*c_undone_moves.moves_set.add(i)).latitude,
                        longtitude: (*c_undone_moves.moves_set.add(i)).longtitude,
                    };
                    undone_moves.push(pos);
                }
            }
            caro_free_move_set(&mut c_undone_moves as *mut CARO_Moves_Set);
        }
        undone_moves
    }
}

impl Drop for SimpleCaro {
    fn drop(&mut self) {
        unsafe {caro_deinit_game(self.gid);}
    }
}