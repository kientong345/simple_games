use std::{collections::HashMap, sync::Arc};
use tokio::{sync::Mutex, task::JoinHandle};

use crate::{
    communication::{self, HandleAction}, id_pool
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
    handler: Arc<Mutex<communication::ClientHandler>>,
    joinhandler: JoinHandle<()>,
}

impl Player {
    async fn new(pid: i32, stream: communication::Stream) -> Self {
        let handler = Arc::new(Mutex::new(communication::ClientHandler::new(stream)));
        let handler_clone = handler.clone();
        let joinhandler = tokio::spawn(communication::ClientHandler::handling_request(handler_clone));
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

    async fn response(&self, message: communication::MessagePacket) {
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
    async fn add_player(&mut self, stream: communication::Stream) -> i32;
    fn remove_player(&mut self, pid: i32);
    fn set_player_state(&mut self, pid: i32, state: PlayerState);
    fn get_player_state(&self, pid: i32) -> Option<PlayerState>;
    async fn set_action_on_request(&mut self, pid: i32, action: HandleAction);
    async fn get_action_on_request(&self, pid: i32) -> HandleAction;
    async fn response(&self, pid: i32, message: communication::MessagePacket);
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
    async fn add_player(&mut self, stream: communication::Stream) -> i32 {
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

    async fn response(&self, pid: i32, message: communication::MessagePacket) {
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

// pub struct PlayerStream {
//     pid: i32,
//     stream: SharedPtr<TcpStream>,
//     state: PlayerState,
//     callback_on_request: SharedPtr<Box<dyn FnMut(MessagePacket)>>,
//     wait_for_req_thread: Option<JoinHandle<()>>,
//     alive_flag: SharedPtr<bool>,
// }

// impl PlayerStream {
//     pub fn new(pid: i32, stream: TcpStream) -> Self {
//         let stream = make_shared!(stream);
//         let stream_clone = stream.clone();
//         let callback: Box<dyn FnMut(MessagePacket)> = Box::new(|msg: MessagePacket| {});
//         let callback_on_request = make_shared!(callback);
//         let callback_clone = callback_on_request.clone();
//         let alive_flag = make_shared!(true);
//         let alive_flag_clone = alive_flag.clone();

//         let handle = thread::spawn(move || {
//             // let mut buf = [0; 256];
//             // while access_shared!(alive_flag_clone) {
//             //     access_shared!(stream_clone).read(&mut buf).unwrap();
//             //     access_shared!(callback_clone)((&buf as &[u8]).to_message_packet());
//             // }
//         });

//         Self {
//             pid,
//             stream,
//             state: PlayerState::Logged(ConnectState::Connected),
//             callback_on_request,
//             wait_for_req_thread: Some(handle),
//             alive_flag,
//         }
//     }

//     pub fn pid(&self) -> i32 {
//         self.pid
//     }

//     pub fn set_state(&mut self, state: PlayerState) {
//         self.state = state;
//     }

//     pub fn get_state(&self) -> PlayerState {
//         self.state.clone()
//     }

//     pub fn set_callback_on_request<T: FnMut(MessagePacket) + 'static>(&mut self, callback: T) {
//         access_shared!(self.callback_on_request) = Box::<T>::new(callback);
//     }

//     pub fn get_callback_on_request(&self) -> SharedPtr<Box<dyn FnMut(MessagePacket)>> {
//         self.callback_on_request.clone()
//     }

//     pub fn response(&self, message: MessagePacket) {

//     }

//     pub fn check_alive(&mut self) -> bool {
//         todo!()
//     }

// }

// impl Drop for PlayerStream {
//     fn drop(&mut self) {
//         access_shared!(self.alive_flag) = false;
//         if let Some(handle) = self.wait_for_req_thread.take() {
//             handle.join().unwrap();
//         }
//     }
// }

// pub struct PlayerManager {
//     players_map: HashMap<i32, SharedPtr<PlayerStream>>,
//     max_player: usize,
//     pid_pool: IdPool,
// }

// impl PlayerManager {
//     pub fn new(max_player: usize) -> Self {
//         Self {
//             players_map: HashMap::new(),
//             max_player,
//             pid_pool: IdPool::new(),
//         }
//     }

//     pub fn add_player(&mut self, stream: TcpStream) -> i32 {
//         if self.players_map.len() >= self.max_player {
//             return -1;
//         }
//         let pid = self.pid_pool.alloc_id();
//         let new_player = make_shared!(PlayerStream::new(pid, stream));
//         self.players_map.insert(pid, new_player);
//         pid
//     }

//     pub fn remove_player(&mut self, pid: i32) {
//         self.pid_pool.dealloc_id(pid);
//         self.players_map.remove(&pid);
//     }

//     pub fn get_player(&mut self, pid: i32) -> Option<SharedPtr<PlayerStream>> {
//         Some(self.players_map.get(&pid).unwrap().clone())
//     }

//     pub fn is_player_exist(&self, pid: i32) -> bool {
//         self.players_map.contains_key(&pid)
//     }

// }