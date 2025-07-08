use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

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
    responser: client_handler::Responser,
    request_getter: Arc<Mutex<client_handler::RequestGetter>>,
    response_handler: Option<Arc<Mutex<client_handler::ResponseHandler>>>,
}

impl Player {
    fn new(pid: i32, receiver: client_handler::Receiver, sender: client_handler::Sender) -> Self {
        let responser = client_handler::Responser::new(sender);
        let request_getter = Arc::new(Mutex::new(client_handler::RequestGetter::new(receiver)));
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

    async fn set_action_on_request(&mut self, action: HandleAction) {
        self.request_getter.lock().await.set_action_on_request(action);
    }

    async fn get_action_on_request(&self) -> HandleAction {
        self.request_getter.lock().await.get_action_on_request()
    }

    async fn response(&mut self, message: caro_protocol::MessagePacket) {
        self.responser.send_response(message).await;
    }

    async fn handling_request(&mut self) -> bool {
        if self.response_handler.is_none() {
            self.response_handler = Some(Arc::new(Mutex::new(client_handler::RequestGetter::handling_request(self.request_getter.clone()).await)));
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
            client_handler::RequestGetter::stop_handling_request(request_handler_clone).await;
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

pub trait PlayerManager {
    fn add_player(&mut self, receiver: client_handler::Receiver, sender: client_handler::Sender) -> i32;
    fn remove_player(&mut self, pid: i32);
    fn set_player_state(&mut self, pid: i32, state: PlayerState);
    fn get_player_state(&self, pid: i32) -> Option<PlayerState>;
    async fn set_action_on_request(&mut self, pid: i32, action: HandleAction);
    async fn get_action_on_request(&self, pid: i32) -> HandleAction;
    async fn handling_request(&mut self, pid: i32) -> bool;
    async fn stop_handling_request(&mut self, pid: i32) -> bool;
    async fn response(&mut self, pid: i32, message: caro_protocol::MessagePacket);
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
    fn add_player(&mut self, receiver: client_handler::Receiver, sender: client_handler::Sender) -> i32 {
        if self.players_map.len() >= self.max_player {
            return -1;
        }
        let pid = self.pid_pool.alloc_id();
        let new_player = Player::new(pid, receiver, sender);
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

    async fn handling_request(&mut self, pid: i32) -> bool {
        if let Some(player) = self.players_map.get_mut(&pid) {
            return player.handling_request().await;
        }
        false
    }

    async fn stop_handling_request(&mut self, pid: i32) -> bool {
        if let Some(player) = self.players_map.get_mut(&pid) {
            return player.stop_handling_request().await;
        }
        false
    }

    async fn response(&mut self, pid: i32, message: caro_protocol::MessagePacket) {
        if let Some(player) = self.players_map.get_mut(&pid) {
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
