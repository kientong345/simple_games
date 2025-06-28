use std::{collections::HashMap, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    client_handler::{self, HandleAction}, id_pool, caro_protocol
};

#[derive(Debug, Clone, Copy)]
pub enum ConnectState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerState {
    Logged(ConnectState),
    Waiting(ConnectState),
    InGame(ConnectState),
}

struct Player {
    pid: i32,
    state: PlayerState,
    handler: Arc<Mutex<client_handler::ClientHandler>>,
    joinhandler: JoinHandle<()>,
}

impl Player {
    async fn new(pid: i32, stream: client_handler::Stream) -> Self {
        let handler = Arc::new(Mutex::new(client_handler::ClientHandler::new(stream)));
        let handler_clone = handler.clone();
        let joinhandler = tokio::spawn(client_handler::ClientHandler::handling_request(handler_clone));
        Self {
            pid,
            state: PlayerState::Logged(ConnectState::Connected),
            handler,
            joinhandler,
        }
    }

    fn pid(&self) -> i32 {
        self.pid
    }

    fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    fn get_state(&self) -> PlayerState {
        self.state
    }

    async fn set_action_on_request(&mut self, action: HandleAction) {
        self.handler.lock().await.set_action_on_request(action).await;
    }

    async fn get_action_on_request(&self) -> HandleAction {
        self.handler.lock().await.get_action_on_request().await
    }

    async fn response(&self, message: caro_protocol::MessagePacket) {
        self.handler.lock().await.response(message).await;
    }

    fn check_alive(&self) -> bool {
        // Here you would typically check if the stream is still open
        // For example:
        // self.stream.peek(&mut [0; 1]).is_ok()
        true // Placeholder for actual implementation
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        self.joinhandler.abort();
    }
}

pub trait PlayerManager {
    async fn add_player(&mut self, stream: client_handler::Stream) -> i32;
    fn remove_player(&mut self, pid: i32);
    fn set_player_state(&mut self, pid: i32, state: PlayerState);
    fn get_player_state(&self, pid: i32) -> Option<PlayerState>;
    async fn set_action_on_request(&mut self, pid: i32, action: HandleAction);
    async fn get_action_on_request(&self, pid: i32) -> HandleAction;
    async fn response(&self, pid: i32, message: caro_protocol::MessagePacket);
    fn check_alive(&self, pid: i32) -> bool;
    fn player_exist(&self, pid: i32) -> bool;
}

pub struct PlayerContainer {
    players_map: HashMap<i32, Player>,
    max_player: usize,
    pid_pool: id_pool::IdPool,
}

impl PlayerContainer {
    pub fn new(max_player: usize, pid_pool: id_pool::IdPool) -> Self {
        Self {
            players_map: HashMap::new(),
            max_player,
            pid_pool,
        }
    }
}

impl PlayerManager for PlayerContainer {
    async fn add_player(&mut self, stream: client_handler::Stream) -> i32 {
        if self.players_map.len() >= self.max_player {
            return -1;
        }
        let pid = self.pid_pool.alloc_id();
        let new_player = Player::new(pid, stream).await;
        self.players_map.insert(pid, new_player);
        pid
    }

    fn remove_player(&mut self, pid: i32) {
        self.pid_pool.dealloc_id(pid);
        self.players_map.remove(&pid);
    }

    fn set_player_state(&mut self, pid: i32, state: PlayerState) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.set_state(state);
        }
    }

    fn get_player_state(&self, pid: i32) -> Option<PlayerState> {
        self.players_map.get(&pid).map(|p| p.get_state())
    }

    async fn set_action_on_request(&mut self, pid: i32, action: HandleAction) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.set_action_on_request(action).await;
        }
    }

    async fn get_action_on_request(&self, pid: i32) -> HandleAction {
        self.players_map.get(&pid).unwrap().get_action_on_request().await
    }

    async fn response(&self, pid: i32, message: caro_protocol::MessagePacket) {
        if let Some(player) = self.players_map.get(&pid) {
            player.response(message).await;
        }
    }

    fn check_alive(&self, pid: i32) -> bool {
        if let Some(player) = self.players_map.get(&pid) {
            player.check_alive()
        } else {
            false
        }
    }

    fn player_exist(&self, pid: i32) -> bool {
        self.players_map.contains_key(&pid)
    }
}
