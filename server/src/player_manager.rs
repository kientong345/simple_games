use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::{
    server_endpoint,
    id_pool,
    caro_protocol
};

struct Player {
    state: caro_protocol::PlayerState,
    responser: server_endpoint::Responser,
    request_getter: Arc<RwLock<server_endpoint::RequestGetter>>,
    response_handler: Option<Arc<RwLock<server_endpoint::ResponseHandler>>>,

    responsed_to_checkalive: bool,
}

impl Player {
    fn new(receiver: server_endpoint::Receiver, sender: server_endpoint::Sender) -> Self {
        let responser = server_endpoint::Responser::new(sender);
        let request_getter = Arc::new(RwLock::new(server_endpoint::RequestGetter::new(receiver)));
        Self {
            state: caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected),
            responser,
            request_getter,
            response_handler: None,
            responsed_to_checkalive: false,
        }
    }

    fn set_state(&mut self, state: caro_protocol::PlayerState) {
        self.state = state;
    }

    fn get_state(&self) -> caro_protocol::PlayerState {
        self.state
    }

    async fn set_action_on_request(&mut self, action: server_endpoint::HandleAction) {
        self.request_getter.write().await.set_action_on_request(action);
    }

    async fn get_action_on_request(&self) -> server_endpoint::HandleAction {
        self.request_getter.read().await.get_action_on_request()
    }

    async fn response(&mut self, message: caro_protocol::MessagePacket) {
        tokio::time::sleep(std::time::Duration::from_millis(2)).await; // avoid bombarding the client with messages
        self.responser.send_response(message).await;
    }

    async fn handling_request(&mut self) -> bool {
        if self.response_handler.is_none() {
            self.response_handler = Some(Arc::new(RwLock::new(server_endpoint::RequestGetter::handling_request(self.request_getter.clone()).await)));
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

    async fn send_checkalive_message(&mut self) {
        self.responsed_to_checkalive = false;
        let code = caro_protocol::ServerCode::AreYouAlive;
        let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
        self.response(new_packet).await;
    }

    fn mark_as_responsed_to_checkalive(&mut self) {
        self.responsed_to_checkalive = true;
    }

    fn is_responsed_to_checkalive(&self) -> bool {
        self.responsed_to_checkalive
    }
}

pub struct PlayerContainer {
    players_map: HashMap<caro_protocol::PlayerId, Player>,
    max_player: usize,
    pid_pool: id_pool::IdPool<i32>,
}

impl PlayerContainer {
    pub fn new(max_player: usize, pid_pool: id_pool::IdPool<i32>) -> Self {
        Self {
            players_map: HashMap::new(),
            max_player,
            pid_pool,
        }
    }
}

impl PlayerContainer {
    pub fn add_player(&mut self, receiver: server_endpoint::Receiver, sender: server_endpoint::Sender) -> caro_protocol::PlayerId {
        if self.players_map.len() >= self.max_player {
            return -1;
        }
        let pid = self.pid_pool.alloc_id();
        let new_player = Player::new(receiver, sender);
        self.players_map.insert(pid, new_player);
        pid
    }

    pub fn remove_player(&mut self, pid: caro_protocol::PlayerId) {
        self.pid_pool.dealloc_id(pid);
        self.players_map.remove(&pid);
    }

    pub fn set_player_state(&mut self, pid: caro_protocol::PlayerId, state: caro_protocol::PlayerState) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.set_state(state);
        }
    }

    pub fn get_player_state(&self, pid: caro_protocol::PlayerId) -> Option<caro_protocol::PlayerState> {
        self.players_map.get(&pid).map(|p| p.get_state())
    }

    pub async fn set_action_on_request(&mut self, pid: caro_protocol::PlayerId, action: server_endpoint::HandleAction) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.set_action_on_request(action).await;
        }
    }

    pub async fn get_action_on_request(&self, pid: caro_protocol::PlayerId) -> server_endpoint::HandleAction {
        self.players_map.get(&pid).unwrap().get_action_on_request().await
    }

    pub async fn handling_request(&mut self, pid: caro_protocol::PlayerId) -> bool {
        if let Some(player) = self.players_map.get_mut(&pid) {
            return player.handling_request().await;
        }
        false
    }

    pub async fn stop_handling_request(&mut self, pid: caro_protocol::PlayerId) -> bool {
        if let Some(player) = self.players_map.get_mut(&pid) {
            return player.stop_handling_request().await;
        }
        false
    }

    pub async fn response(&mut self, pid: caro_protocol::PlayerId, message: caro_protocol::MessagePacket) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.response(message).await;
        }
    }

    pub fn player_exist(&self, pid: caro_protocol::PlayerId) -> bool {
        self.players_map.contains_key(&pid)
    }

    pub async fn send_checkalive_message(&mut self, pid: caro_protocol::PlayerId) -> bool {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.send_checkalive_message().await;
            true
        } else {
            false
        }
    }

    pub fn mark_as_responsed_to_checkalive(&mut self, pid: caro_protocol::PlayerId) {
        if let Some(player) = self.players_map.get_mut(&pid) {
            player.mark_as_responsed_to_checkalive();
        }
    }

    pub fn is_responsed_to_checkalive(&self, pid: caro_protocol::PlayerId) -> bool {
        if let Some(player) = self.players_map.get(&pid) {
            player.is_responsed_to_checkalive()
        } else {
            false
        }
    }


}
