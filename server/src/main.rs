use std::sync::Arc;
use tokio::sync::Mutex;

use simple_caro_app::{
    caro_protocol, client_handler, game_manager, id_pool, make_action, player_manager::{self, PlayerManager}, room_manager::{self, RoomManager}
};

#[tokio::main]
async fn main() {
    let pid_pool = id_pool::IdPool::new();
    let player_manager = Arc::new(Mutex::new(player_manager::PlayerContainer::new(256, pid_pool)));
    let rid_pool = id_pool::IdPool::new();
    let room_manager = Arc::new(Mutex::new(room_manager::RoomContainer::new(256, rid_pool)));

    // let mut player_tracker = PlayerTracker::new(player_manager.clone());

    let mut listener = client_handler::Listener::new(caro_protocol::SERVER_ADDRESS).await;

    while let (receiver, sender) = listener.accept().await {
        let new_pid = player_manager.lock().await.add_player(receiver, sender);
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
                    println!("{:?}", msg.code());
                    let room_id = handle_room_request(player_manager.clone(), room_manager.clone(), new_pid, msg).await;
                    if room_manager.lock().await.room_full(room_id) {
                        let (pid1, pid2) =room_manager.lock().await.get_pids_in_room(room_id).unwrap();
                        let code = caro_protocol::ServerCode::YourRoomIsFull(room_id);
                        let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                        tokio::time::sleep(std::time::Duration::from_millis(2)).await;
                        player_manager.lock().await.response(pid1, new_packet.clone()).await;
                        tokio::time::sleep(std::time::Duration::from_millis(2)).await;
                        player_manager.lock().await.response(pid2, new_packet).await;
                        run_a_game(player_manager.clone(), room_manager.clone(), room_id).await;
                    }
                };
                Box::pin(future) as futures::future::BoxFuture<'static, ()>
            }
        )).await;

        player_manager.lock().await.handling_request(new_pid).await;
    }
}



async fn handle_room_request(player_manager: Arc<Mutex<player_manager::PlayerContainer>>, room_manager: Arc<Mutex<room_manager::RoomContainer>>,
                            pid: i32, message: caro_protocol::MessagePacket) -> i32 {
    let mut room_id = -1;

    if let caro_protocol::GenericCode::Player(player_code) = message.code() {
        match player_code {
            caro_protocol::PlayerCode::RequestRoomAsPlayer1(rule_type) => {
                let new_rid = room_manager.lock().await.add_room(rule_type);
                if new_rid == -1 {
                    let code = caro_protocol::ServerCode::FailedToCreateRoom;
                    let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                    player_manager.lock().await.response(pid, new_packet).await;
                    return -1;
                }
                let _result = room_manager.lock().await.add_player_to_room(new_rid, room_manager::PlayerOrder::Player1(pid));
                if !_result {
                    let code = caro_protocol::ServerCode::FailedToJoinRoom(new_rid);
                    let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                    player_manager.lock().await.response(pid, new_packet).await;
                    return -1;
                }
                player_manager.lock().await.set_player_state(pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                let code = caro_protocol::ServerCode::JoinedRoomAsPlayer1(new_rid);
                let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                player_manager.lock().await.response(pid, new_packet).await;
                room_id = new_rid;
            },
            caro_protocol::PlayerCode::JoinRoomAsPlayer2(rid) => {
                let _result = room_manager.lock().await.add_player_to_room(rid, room_manager::PlayerOrder::Player2(pid));
                player_manager.lock().await.set_player_state(pid, player_manager::PlayerState::Waiting(player_manager::ConnectState::Connected));
                let code = caro_protocol::ServerCode::JoinedRoomAsPlayer2(rid);
                let new_packet = caro_protocol::MessagePacket::new_server_packet(code);
                player_manager.lock().await.response(pid, new_packet).await;
                room_id = rid;
            },
            _ => {
                // do not process other requests
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