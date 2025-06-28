use std::collections::HashMap;

use crate::{
    id_pool::IdPool, protocol
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
    rule: protocol::GameRule,
}

impl GameRoom {
    fn new(rid: i32, rule: protocol::GameRule) -> Self {
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

    fn get_rule(&self) -> protocol::GameRule {
        self.rule
    }
}

pub trait RoomManager {
    fn add_room(&mut self, rule: protocol::GameRule) -> i32;
    fn remove_room(&mut self, rid: i32);
    fn add_player_to_room(&mut self, rid: i32, player: PlayerOrder) -> bool;
    fn remove_player_from_room(&mut self, rid: i32, player: PlayerOrder) -> bool;
    fn get_pids_in_room(&self, rid: i32) -> Option<(i32, i32)>;
    fn get_rule_in_room(&self, rid: i32) -> Option<protocol::GameRule>;
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
    fn add_room(&mut self, rule: protocol::GameRule) -> i32 {
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

    fn get_rule_in_room(&self, rid: i32) -> Option<protocol::GameRule> {
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
