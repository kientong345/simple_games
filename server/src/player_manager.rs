use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use crate::{
    server_endpoint,
    id_pool,
    caro_protocol
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
    responser: server_endpoint::Responser,
    request_getter: Arc<Mutex<server_endpoint::RequestGetter>>,
    response_handler: Option<Arc<Mutex<server_endpoint::ResponseHandler>>>,
}

impl Player {
    fn new(pid: i32, receiver: server_endpoint::Receiver, sender: server_endpoint::Sender) -> Self {
        let responser = server_endpoint::Responser::new(sender);
        let request_getter = Arc::new(Mutex::new(server_endpoint::RequestGetter::new(receiver)));
        Self {
            pid,
            state: PlayerState::Logged(ConnectState::Connected),
            responser,
            request_getter,
            response_handler: None,
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

    async fn set_action_on_request(&mut self, action: server_endpoint::HandleAction) {
        self.request_getter.lock().await.set_action_on_request(action);
    }

    async fn get_action_on_request(&self) -> server_endpoint::HandleAction {
        self.request_getter.lock().await.get_action_on_request()
    }

    async fn response(&mut self, message: caro_protocol::MessagePacket) {
        tokio::time::sleep(std::time::Duration::from_millis(2)).await; // avoid bombarding the client with messages
        self.responser.send_response(message).await;
    }

    async fn handling_request(&mut self) -> bool {
        if self.response_handler.is_none() {
            self.response_handler = Some(Arc::new(Mutex::new(server_endpoint::RequestGetter::handling_request(self.request_getter.clone()).await)));
            true
        } else {
            false
        }
    }

    async fn stop_handling_request(&mut self) -> bool {
        if self.response_handler.is_none() {
            false
        } else {
            let request_handler_clone = self.response_handler.clone().unwrap();
            server_endpoint::RequestGetter::stop_handling_request(request_handler_clone).await;
            true
        }
    }

    fn check_alive(&self) -> bool {
        // Here you would typically check if the stream is still open
        // For example:
        // self.stream.peek(&mut [0; 1]).is_ok()
        true // Placeholder for actual implementation
    }
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

impl PlayerContainer {
    pub fn add_player(&mut self, receiver: server_endpoint::Receiver, sender: server_endpoint::Sender) -> i32 {
        if self.players_map.len() >= self.max_player {
            return -1;
        }
        let pid = self.pid_pool.alloc_id();
        let new_player = Player::new(pid, receiver, sender);
        self.players_map.insert(pid, new_player);
        pid
    }

    pub fn remove_player(&mut self, pid: i32) {
        self.pid_pool.dealloc_id(pid);
        self.players_map.remove(&pid);
    }

    pub fn set_player_state(&mut self, pid: i32, state: PlayerState) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.set_state(state);
        }
    }

    pub fn get_player_state(&self, pid: i32) -> Option<PlayerState> {
        self.players_map.get(&pid).map(|p| p.get_state())
    }

    pub async fn set_action_on_request(&mut self, pid: i32, action: server_endpoint::HandleAction) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.set_action_on_request(action).await;
        }
    }

    pub async fn get_action_on_request(&self, pid: i32) -> server_endpoint::HandleAction {
        self.players_map.get(&pid).unwrap().get_action_on_request().await
    }

    pub async fn handling_request(&mut self, pid: i32) -> bool {
        if let Some(player) = self.players_map.get_mut(&pid) {
            return player.handling_request().await;
        }
        false
    }

    pub async fn stop_handling_request(&mut self, pid: i32) -> bool {
        if let Some(player) = self.players_map.get_mut(&pid) {
            return player.stop_handling_request().await;
        }
        false
    }

    pub async fn response(&mut self, pid: i32, message: caro_protocol::MessagePacket) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.response(message).await;
        }
    }

    pub fn check_alive(&self, pid: i32) -> bool {
        if let Some(player) = self.players_map.get(&pid) {
            player.check_alive()
        } else {
            false
        }
    }

    pub fn player_exist(&self, pid: i32) -> bool {
        self.players_map.contains_key(&pid)
    }
}
