use std::sync::Arc;

use tokio::sync::RwLock;

use crate::player_manager;

pub struct PlayerTracker {
    player_manager: Arc<RwLock<player_manager::PlayerContainer>>,
}

impl PlayerTracker {
    pub fn new(player_manager: Arc<RwLock<player_manager::PlayerContainer>>) -> Self {
        Self {
            player_manager,
        }
    }
}

// pub struct PlayerTracker {
//     player_manager: Arc<Mutex<PlayerManager>>,
//     track_list: Arc<Mutex<HashMap<i32, (fn(), fn())>>>,
//     tracking_thread: JoinHandle<()>,
// }

// impl PlayerTracker {
//     pub fn new(player_manager: Arc<Mutex<PlayerManager>>) -> Self {
//         let player_manager_clone = player_manager.clone();
//         let track_list = make_shared!(HashMap::new());
//         let track_list_clone = track_list.clone();

//         let tracking_thread = thread::spawn(move || {
//         //     loop {
//         //         let mut result_table = HashMap::<i32, Pin<Box<dyn Future<Output = bool> + Send>>>::new();
//         //         for (key, _val) in access_shared!(track_list_clone).iter() {
//         //             let player = access_shared!(player_manager_clone).get_player(*key).unwrap();
//         //             // let check_result = async {
//         //             //     access_shared!(player).check_alive().await
//         //             // };
//         //             // result_table.insert(*key, Box::pin(check_result));
//         //         }

//         //         std::thread::sleep(std::time::Duration::from_secs(10));

//         //         for (key, val) in result_table.iter() {
//         //             let player = access_shared!(player_manager_clone).get_player(*key).unwrap();
//         //             // if val == false && !(access_shared!(player).get_state() == PlayerState::Disconnected) {
//         //             //     let (fn1, fn2) = *(access_shared!(track_list_clone).get(key).unwrap());
//         //             //     fn1();
//         //             //     access_shared!(player).set_state(PlayerState::Disconnected);
//         //             // } else if val == true && (access_shared!(player).get_state() == PlayerState::Disconnected) {
//         //             //     let (fn1, fn2) = *(access_shared!(track_list_clone).get(key).unwrap());
//         //             //     fn2();
//         //             //     access_shared!(player).set_state(PlayerState::Disconnected);
//         //             // }
//         //         }
//         //     }
//         });

//         Self {
//             player_manager,
//             track_list,
//             tracking_thread,
//         }
//     }

//     pub fn track_player(&mut self, pid: i32, callback_when_disconnect: fn(), callback_when_reconnect: fn()) {
//         let mut list = self.track_list.lock().unwrap();
//         list.insert(pid, (callback_when_disconnect, callback_when_reconnect));
//     }

// }