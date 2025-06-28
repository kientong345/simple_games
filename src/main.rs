use std::sync::Arc;
use tokio::sync::Mutex;

use simple_caro_app::{
    client_handler, game_manager, id_pool, make_action, player_manager::{self, PlayerManager}, caro_protocol, room_manager::{self, RoomManager}
};

#[tokio::main]
async fn main() {
    let pid_pool = id_pool::IdPool::new();
    let player_manager = Arc::new(Mutex::new(player_manager::PlayerContainer::new(256, pid_pool)));
    let rid_pool = id_pool::IdPool::new();
    let room_manager = Arc::new(Mutex::new(room_manager::RoomContainer::new(256, rid_pool)));

    // let mut player_tracker = PlayerTracker::new(player_manager.clone());

    let mut listener = client_handler::Listener::new(client_handler::SERVER_ADDRESS).await;

    while let stream = listener.accept().await {
        let new_pid = player_manager.lock().await.add_player(stream).await;
        player_manager.lock().await.set_player_state(new_pid, player_manager::PlayerState::Logged(player_manager::ConnectState::Connected));
        // player_tracker.track_player(pid, move || {
        //     todo!()
        // }, move || {
        //     todo!()
        // });

        let player_manager_clone = player_manager.clone();
        let room_manager_clone = room_manager.clone();

        player_manager.lock().await.set_action_on_request(
            new_pid,
            make_action!(move |msg: caro_protocol::MessagePacket| {
                let player_manager = player_manager_clone.clone();
                let room_manager = room_manager_clone.clone();
                let future = async move {
                    // println!("{:?}", msg.command());
                    // println!("{:?}", msg.to_serial());
                    let room_id = handle_room_request(player_manager.clone(), room_manager.clone(), new_pid, msg).await;
                    run_a_game(player_manager.clone(), room_manager.clone(), room_id).await;
                };
                Box::pin(future) as futures::future::BoxFuture<'static, ()>
            }
        )).await;
    }
}

async fn handle_room_request(player_manager: Arc<Mutex<player_manager::PlayerContainer>>, room_manager: Arc<Mutex<room_manager::RoomContainer>>,
                            pid: i32, message: caro_protocol::MessagePacket) -> i32 {
    let mut room_id = -1;

    if let caro_protocol::GenericCode::Player(player_code) = message.code() {
        match player_code {
            caro_protocol::PlayerCode::RequestRoomAsPlayer1(rule_type) => {
                let new_rid = room_manager.lock().await.add_room(rule_type);
                let _result = room_manager.lock().await.add_player_to_room(new_rid, room_manager::PlayerOrder::Player2(pid));
                player_manager.lock().await.set_player_state(pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                // player_manager_clone.lock().await.response(new_pid, new_rid);
                room_id = new_rid;
            },
            caro_protocol::PlayerCode::JoinRoomAsPlayer2(rid) => {
                let _result = room_manager.lock().await.add_player_to_room(rid, room_manager::PlayerOrder::Player2(pid));
                player_manager.lock().await.set_player_state(pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                // player_manager_clone.lock().unwrap().response(new_pid, result);
                room_id = rid;
            },
            _ => {
                // do nothing
            }
        }
    }
    
    room_id
}

async fn run_a_game(player_manager: Arc<Mutex<player_manager::PlayerContainer>>, room_manager: Arc<Mutex<room_manager::RoomContainer>>,
                    rid: i32) {
    if room_manager.lock().await.room_full(rid) {
        let mut game_operator = game_manager::GameOperator::new(player_manager.clone(), room_manager.clone());
        match game_operator.try_operate_in_room(rid).await {
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
    }
}