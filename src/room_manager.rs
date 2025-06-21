use std::collections::HashMap;

use crate::{
    communication, id_pool::IdPool
};

#[derive(Debug, Clone, Copy)]
pub enum PlayerOrder {
    Player1(i32),
    Player2(i32),
}

#[derive(Debug, Clone, Copy)]
pub enum RoomType {
    TicTacToe,
    FourBlockOne,
    FiveBlockTwo,
}

struct GameRoom {
    rid: i32,
    player1_id: i32,
    player2_id: i32,
    rule: communication::GameRule,
}

impl GameRoom {
    fn new(rid: i32, rule: communication::GameRule) -> Self {
        Self {
            rid,
            player1_id: -1,
            player2_id: -1,
            rule,
        }
    }

    fn rid(&self) -> i32 {
        self.rid
    }

    fn is_full(&self) -> bool {
        self.player1_id != -1 && self.player2_id != -1
    }

    fn add_player(&mut self, player: PlayerOrder) {
        match player {
            PlayerOrder::Player1(pid) => self.player1_id = pid,
            PlayerOrder::Player2(pid) => self.player2_id = pid,
        }
    }

    fn remove_player(&mut self, player: PlayerOrder) {
        match player {
            PlayerOrder::Player1(_) => self.player1_id = -1,
            PlayerOrder::Player2(_) => self.player2_id = -1,
        }
    }

    fn get_pids(&self) -> (i32, i32) {
        (self.player1_id, self.player2_id)
    }

    fn get_rule(&self) -> communication::GameRule {
        self.rule
    }
}

pub trait RoomManager {
    fn add_room(&mut self, rule: communication::GameRule) -> i32;
    fn remove_room(&mut self, rid: i32);
    fn add_player_to_room(&mut self, rid: i32, player: PlayerOrder) -> bool;
    fn remove_player_from_room(&mut self, rid: i32, player: PlayerOrder) -> bool;
    fn get_pids_in_room(&self, rid: i32) -> Option<(i32, i32)>;
    fn get_rule_in_room(&self, rid: i32) -> Option<communication::GameRule>;
    fn room_full(&self, rid: i32) -> bool;
    fn room_exist(&self, rid: i32) -> bool;
}

pub struct RoomContainer {
    rooms_set: HashMap<i32, GameRoom>,
    max_rooms: usize,
    rid_pool: IdPool,
}

impl RoomContainer {
    pub fn new(max_rooms: usize, rid_pool: IdPool) -> Self {
        Self {
            rooms_set: HashMap::new(),
            max_rooms,
            rid_pool,
        }
    }
}

impl RoomManager for RoomContainer {
    fn add_room(&mut self, rule: communication::GameRule) -> i32 {
        if self.rooms_set.len() >= self.max_rooms {
            return -1;
        }
        let new_rid = self.rid_pool.alloc_id();
        let new_room = GameRoom::new(new_rid, rule);
        self.rooms_set.insert(new_rid, new_room);
        new_rid
    }

    fn remove_room(&mut self, rid: i32) {
        self.rid_pool.dealloc_id(rid);
        self.rooms_set.remove(&rid);
    }

    fn add_player_to_room(&mut self, rid: i32, player: PlayerOrder) -> bool {
        if let Some(room) = self.rooms_set.get_mut(&rid) {
            room.add_player(player);
            true
        } else {
            false
        }
    }

    fn remove_player_from_room(&mut self, rid: i32, player: PlayerOrder) -> bool {
        if let Some(room) = self.rooms_set.get_mut(&rid) {
            room.remove_player(player);
            true
        } else {
            false
        }
    }

    fn get_pids_in_room(&self, rid: i32) -> Option<(i32, i32)> {
        if let Some(room) = self.rooms_set.get(&rid) {
            Some(room.get_pids())
        } else {
            None
        }
    }

    fn get_rule_in_room(&self, rid: i32) -> Option<communication::GameRule> {
        if let Some(room) = self.rooms_set.get(&rid) {
            Some(room.get_rule())
        } else {
            None
        }
    }

    fn room_full(&self, rid: i32) -> bool {
        if let Some(room) = self.rooms_set.get(&rid) {
            room.is_full()
        } else {
            false
        }
    }

    fn room_exist(&self, rid: i32) -> bool {
        self.rooms_set.contains_key(&rid)
    }
}

// pub struct GameRoom {
//     rid: i32,
//     player_list: [Option<SharedPtr<PlayerStream>>; 2],
//     game_manager: Option<SharedPtr<GameManager>>,
//     room_type: RoomType,
// }

// impl GameRoom {
//     pub fn new(rid: i32, room_type: RoomType) -> Self {
//         Self {
//             rid,
//             player_list: [None, None],
//             game_manager: None,
//             room_type,
//         }
//     }

//     pub fn rid(&self) -> i32 {
//         self.rid
//     }

//     pub fn is_full(&mut self) -> bool {
//         self.player_list[0].is_some() && self.player_list[1].is_some()
//     }

//     pub fn add_player(&mut self, player: PlayerOrder) {
//         match player {
//             PlayerOrder::Player1(x) => self.player_list[0] = Some(x),
//             PlayerOrder::Player2(x) => self.player_list[1] = Some(x),
//         }
//     }

//     pub fn remove_player(&mut self, player: PlayerOrder) {
//         match player {
//             PlayerOrder::Player1(_) => self.player_list[0] = None,
//             PlayerOrder::Player2(_) => self.player_list[1] = None,
//         }
//     }

//     pub fn try_start(&mut self) -> bool {
//         if self.game_manager.is_some() || !self.is_full() {
//             return false;
//         }
        
//         let rule_type = match self.room_type {
//             RoomType::TicTacToe => RuleType::TicTacToe,
//             RoomType::FourBlockOne => RuleType::FourBlockOne,
//             RoomType::FiveBlockTwo => RuleType::FiveBlockTwo,
//         };
//         self.game_manager = Some(make_shared!(GameManager::new(
//             self.player_list[0].clone().unwrap(),
//             self.player_list[0].clone().unwrap(),
//             rule_type,
//         )));

//         let player_clone = self.player_list[0].clone().unwrap();

//         let game_manager_clone = self.game_manager.clone().unwrap();

//         let endgame_flag = Arc::new((Mutex::new(false), Condvar::new()));

//         let endgame_flag_clone = endgame_flag.clone();

//         let prev_player1_callback = access_shared!(player_clone).get_callback_on_request();

//         access_shared!(player_clone).set_callback_on_request(move |msg| {
//             access_shared!(game_manager_clone).execute_player_command(msg.command());
//             if access_shared!(game_manager_clone).is_over() {
//                 let (lock, cvar) = &*endgame_flag_clone;
//                 let mut done = lock.lock().unwrap();
//                 *done = true;
//                 cvar.notify_all();
//             }
//         });

//         let player_clone = self.player_list[1].clone().unwrap();

//         let game_manager_clone = self.game_manager.clone().unwrap();

//         let endgame_flag_clone = endgame_flag.clone();

//         let prev_player2_callback = access_shared!(player_clone).get_callback_on_request();

//         access_shared!(player_clone).set_callback_on_request(move |msg| {
//             access_shared!(game_manager_clone).execute_player_command(msg.command());
//             if access_shared!(game_manager_clone).is_over() {
//                 let (lock, cvar) = &*endgame_flag_clone;
//                 let mut done = lock.lock().unwrap();
//                 *done = true;
//                 cvar.notify_all();
//             }
//         });

//         let game_manager_clone = self.game_manager.clone().unwrap();

//         access_shared!(game_manager_clone).start(GameState::Player1Turn);

//         let (lock, cvar) = &*endgame_flag;
//         let mut done = lock.lock().unwrap();
//         while !*done {
//             done = cvar.wait(done).unwrap();
//         }
//         access_shared!(game_manager_clone).stop();

//         let player_clone = self.player_list[0].clone().unwrap();

//         // access_shared!(player_clone).set_callback_on_request(access_shared!(prev_player1_callback));

//         true

//     }
// }

// pub struct RoomManager {
//     rooms_set: HashMap<i32, SharedPtr<GameRoom>>,
//     max_rooms: usize,
//     rid_pool: IdPool,
// }

// impl RoomManager {
//     pub fn new(max_rooms: usize) -> Self {
//         Self {
//             rooms_set: HashMap::new(),
//             max_rooms,
//             rid_pool: IdPool::new(),
//         }
//     }

//     pub fn create_new_room(&mut self, room_type: RoomType) -> i32 {
//         if self.rooms_set.len() >= self.max_rooms {
//             return -1;
//         }
//         let new_rid = self.rid_pool.alloc_id();
//         let new_room: SharedPtr<GameRoom> = make_shared!(GameRoom::new(new_rid, room_type));
//         self.rooms_set.insert(new_rid, new_room);
//         new_rid
//     }

//     pub fn delete_room(&mut self, rid: i32) {
//         self.rid_pool.dealloc_id(rid);
//         self.rooms_set.remove(&rid);
//     }

//     pub fn get_room(&self, rid: i32) -> Option<SharedPtr<GameRoom>> {
//         if !self.id_exist(rid) {
//             return None;
//         }
//         Some(self.rooms_set.get(&rid).unwrap().clone())
//     }

//     pub fn id_exist(&self, rid: i32) -> bool {
//         self.rooms_set.contains_key(&rid)
//     }
// }