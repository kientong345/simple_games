use std::collections::HashMap;

use crate::{
    id_pool,
    caro_protocol
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
    rule: caro_protocol::GameRule,
}

impl GameRoom {
    fn new(rid: i32, rule: caro_protocol::GameRule) -> Self {
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

    fn is_empty(&self) -> bool {
        self.player1_id == -1 && self.player2_id == -1
    }

    fn add_player(&mut self, player: PlayerOrder) {
        match player {
            PlayerOrder::Player1(pid) => self.player1_id = pid,
            PlayerOrder::Player2(pid) => self.player2_id = pid,
        }
    }

    fn remove_player(&mut self, pid: i32) {
        if self.player1_id == pid {
            self.player1_id = -1;
        } else if self.player2_id == pid {
            self.player2_id = -1;
        } else {

        }
    }

    fn get_pids(&self) -> (i32, i32) {
        (self.player1_id, self.player2_id)
    }

    fn get_rule(&self) -> caro_protocol::GameRule {
        self.rule
    }
}

pub struct RoomContainer {
    rooms_set: HashMap<i32, GameRoom>,
    max_rooms: usize,
    rid_pool: id_pool::IdPool,
}

impl RoomContainer {
    pub fn new(max_rooms: usize, rid_pool: id_pool::IdPool) -> Self {
        Self {
            rooms_set: HashMap::new(),
            max_rooms,
            rid_pool,
        }
    }
}

impl RoomContainer {
    pub fn add_room(&mut self, rule: caro_protocol::GameRule) -> i32 {
        if self.rooms_set.len() >= self.max_rooms {
            return -1;
        }
        let new_rid = self.rid_pool.alloc_id();
        let new_room = GameRoom::new(new_rid, rule);
        self.rooms_set.insert(new_rid, new_room);
        new_rid
    }

    pub fn remove_room(&mut self, rid: i32) {
        self.rid_pool.dealloc_id(rid);
        self.rooms_set.remove(&rid);
    }

    pub fn add_player_to_room(&mut self, rid: i32, player: PlayerOrder) -> bool {
        if let Some(room) = self.rooms_set.get_mut(&rid) {
            room.add_player(player);
            true
        } else {
            false
        }
    }

    pub fn remove_player_from_room(&mut self, rid: i32, pid: i32) -> bool {
        if let Some(room) = self.rooms_set.get_mut(&rid) {
            room.remove_player(pid);
            true
        } else {
            false
        }
    }

    pub fn get_pids_in_room(&self, rid: i32) -> Option<(i32, i32)> {
        if let Some(room) = self.rooms_set.get(&rid) {
            Some(room.get_pids())
        } else {
            None
        }
    }

    pub fn get_rule_in_room(&self, rid: i32) -> Option<caro_protocol::GameRule> {
        if let Some(room) = self.rooms_set.get(&rid) {
            Some(room.get_rule())
        } else {
            None
        }
    }

    pub fn room_full(&self, rid: i32) -> bool {
        if let Some(room) = self.rooms_set.get(&rid) {
            room.is_full()
        } else {
            false
        }
    }

    pub fn room_empty(&self, rid: i32) -> bool {
        if let Some(room) = self.rooms_set.get(&rid) {
            room.is_empty()
        } else {
            false
        }
    }

    pub fn room_exist(&self, rid: i32) -> bool {
        self.rooms_set.contains_key(&rid)
    }

    pub fn find_room_contain_player(&self, pid: i32) -> Option<i32> {
        let target = self.rooms_set.iter().find(|&(_rid, room)| {
            let (pid1, pid2) = room.get_pids();
            pid == pid1 || pid == pid2
        });
        if let Some((rid, _room)) = target {
            Some(*rid)
        } else {
            None
        }
    }
}
