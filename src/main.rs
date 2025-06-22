use std::{sync::{Arc, Mutex}};

use simple_caro_app::{
    game_manager,
    id_pool,
    communication,
    player_manager::{self, PlayerManager},
    room_manager::{self, RoomManager},
};

fn main() {
    let pid_pool = id_pool::IdPool::new();
    let player_manager = Arc::new(Mutex::new(player_manager::PlayerContainer::new(256, pid_pool)));
    let rid_pool = id_pool::IdPool::new();
    let room_manager = Arc::new(Mutex::new(room_manager::RoomContainer::new(256, rid_pool)));

    // let mut player_tracker = PlayerTracker::new(player_manager.clone());

    let listener = communication::Listener::new(communication::SERVER_ADDRESS);

    while let stream = listener.accept() {
        let new_pid = player_manager.lock().unwrap().add_player(stream.unwrap());
        player_manager.lock().unwrap().set_player_state(new_pid, player_manager::PlayerState::Logged(player_manager::ConnectState::Connected));
        // player_tracker.track_player(pid, move || {
        //     todo!()
        // }, move || {
        //     todo!()
        // });
        let player_manager_clone = player_manager.clone();
        let room_manager_clone = room_manager.clone();
        player_manager.lock().unwrap().set_action_on_request(new_pid, move |msg| {
            let mut room_id = -1;
            match msg.command() {
                communication::PlayerCommand::RequestRoomAsPlayer1(rule_type) => {
                    // let game_op = game_manager::GameOperator::new(player_manager_clone.clone(), rule_type);
                    let new_rid = room_manager_clone.lock().unwrap().add_room(rule_type);
                    let result = room_manager_clone.lock().unwrap().add_player_to_room(new_rid, room_manager::PlayerOrder::Player2(new_pid));
                    player_manager_clone.lock().unwrap().set_player_state(new_pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                    // player_manager_clone.lock().unwrap().response(new_pid, new_rid);
                    room_id = new_rid;
                },
                communication::PlayerCommand::JoinRoomAsPlayer2(rid) => {
                    let result = room_manager_clone.lock().unwrap().add_player_to_room(rid, room_manager::PlayerOrder::Player2(new_pid));
                    player_manager_clone.lock().unwrap().set_player_state(new_pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                    // player_manager_clone.lock().unwrap().response(new_pid, result);
                    room_id = rid;
                },
                _ => {
                    // do nothing
                }
            }
            if room_manager_clone.lock().unwrap().room_full(room_id) {
                let mut game_operator = game_manager::GameOperator::new(player_manager_clone.clone(), room_manager_clone.clone());
                match game_operator.try_operate_in_room(room_id) {
                    game_manager::OperationResult::RoomNotExist => {
                        todo!()
                    },
                    game_manager::OperationResult::RoomNotFullYet => {
                        todo!()
                    },
                    game_manager::OperationResult::Player1Left => {
                        todo!()
                    },
                    game_manager::OperationResult::Player2Left => {
                        todo!()
                    },
                    game_manager::OperationResult::Successfully(result) => {
                        todo!()
                    },
                }
                // player_manager_clone.lock().unwrap().response(pid1, start_game_message);
                // player_manager_clone.lock().unwrap().response(pid2, start_game_message);
                
                // let run = async {
                //     room_manager_clone.lock().unwrap().run_game(room_id).await;
                // }
                // tokio::spawn(run);
            }
        });
    }
}