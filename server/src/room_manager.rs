use std::collections::HashMap;

use crate::{
    id_pool,
    caro_protocol
};

#[derive(Debug, Clone, Copy)]
pub enum PlayerOrder {
    Player1(caro_protocol::PlayerId),
    Player2(caro_protocol::PlayerId),
}

struct GameRoom {
    player1_id: caro_protocol::PlayerId,
    player2_id: caro_protocol::PlayerId,
    rule: caro_protocol::GameRule,
}

impl GameRoom {
    fn new(rule: caro_protocol::GameRule) -> Self {
        Self {
            player1_id: -1,
            player2_id: -1,
            rule,
        }
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

    fn remove_player(&mut self, pid: caro_protocol::PlayerId) {
        if self.player1_id == pid {
            self.player1_id = -1;
        } else if self.player2_id == pid {
            self.player2_id = -1;
        } else {

        }
    }

    fn get_pids(&self) -> (caro_protocol::PlayerId, caro_protocol::PlayerId) {
        (self.player1_id, self.player2_id)
    }

    fn get_rule(&self) -> caro_protocol::GameRule {
        self.rule
    }
}

pub struct RoomContainer {
    rooms_set: HashMap<caro_protocol::RoomId, GameRoom>,
    max_rooms: usize,
    rid_pool: id_pool::IdPool<i32>,
}

impl RoomContainer {
    pub fn new(max_rooms: usize, rid_pool: id_pool::IdPool<i32>) -> Self {
        Self {
            rooms_set: HashMap::new(),
            max_rooms,
            rid_pool,
        }
    }
}

impl RoomContainer {
    pub fn add_room(&mut self, rule: caro_protocol::GameRule) -> caro_protocol::RoomId {
        if self.rooms_set.len() >= self.max_rooms {
            return -1;
        }
        let new_rid = self.rid_pool.alloc_id();
        let new_room = GameRoom::new(rule);
        self.rooms_set.insert(new_rid, new_room);
        new_rid
    }

    pub fn remove_room(&mut self, rid: caro_protocol::RoomId) {
        self.rid_pool.dealloc_id(rid);
        self.rooms_set.remove(&rid);
    }

    pub fn add_player_to_room(&mut self, rid: caro_protocol::RoomId, player: PlayerOrder) -> bool {
        if let Some(room) = self.rooms_set.get_mut(&rid) {
            room.add_player(player);
            true
        } else {
            false
        }
    }

    pub fn remove_player_from_room(&mut self, rid: caro_protocol::RoomId, pid: caro_protocol::PlayerId) -> bool {
        if let Some(room) = self.rooms_set.get_mut(&rid) {
            room.remove_player(pid);
            true
        } else {
            false
        }
    }

    pub fn get_pids_in_room(&self, rid: caro_protocol::RoomId) -> Option<(caro_protocol::PlayerId, caro_protocol::PlayerId)> {
        if let Some(room) = self.rooms_set.get(&rid) {
            Some(room.get_pids())
        } else {
            None
        }
    }

    pub fn get_rule_in_room(&self, rid: caro_protocol::RoomId) -> Option<caro_protocol::GameRule> {
        if let Some(room) = self.rooms_set.get(&rid) {
            Some(room.get_rule())
        } else {
            None
        }
    }

    pub fn room_full(&self, rid: caro_protocol::RoomId) -> bool {
        if let Some(room) = self.rooms_set.get(&rid) {
            room.is_full()
        } else {
            false
        }
    }

    pub fn room_empty(&self, rid: caro_protocol::RoomId) -> bool {
        if let Some(room) = self.rooms_set.get(&rid) {
            room.is_empty()
        } else {
            false
        }
    }

    pub fn room_exist(&self, rid: caro_protocol::RoomId) -> bool {
        self.rooms_set.contains_key(&rid)
    }

    pub fn find_room_contain_player(&self, pid: caro_protocol::PlayerId) -> Option<caro_protocol::RoomId> {
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
