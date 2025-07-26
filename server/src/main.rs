use std::sync::Arc;

use simple_caro_app::{
    caro_protocol,
    server_endpoint,
    client_request_executor,
    game_manager,
    id_pool,
    make_action,
    player_manager,
    room_manager
};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let pid_pool = id_pool::IdPool::<i32>::new();
    let player_manager = Arc::new(RwLock::new(player_manager::PlayerContainer::new(256, pid_pool)));
    let rid_pool = id_pool::IdPool::<i32>::new();
    let room_manager = Arc::new(RwLock::new(room_manager::RoomContainer::new(256, rid_pool)));
    let gid_pool = id_pool::IdPool::<i32>::new();
    let game_manager = Arc::new(RwLock::new(game_manager::GameContainer::new(256, gid_pool)));

    let command_executor = Arc::new(RwLock::new(client_request_executor::RequestExecutor::new(player_manager.clone(),
                                                                                                                    room_manager.clone(),
                                                                                                                    game_manager.clone())));

    // let mut player_tracker = PlayerTracker::new(player_manager.clone());

    let mut listener = server_endpoint::Listener::new(caro_protocol::SERVER_ADDRESS).await;

    while let (receiver, sender) = listener.accept().await {
        let new_pid = player_manager.write().await.add_player(receiver, sender);
        player_manager.write().await.set_player_state(new_pid, caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Connected));

        let executor_clone = command_executor.clone();

        player_manager.write().await.set_action_on_request(
            new_pid,
            make_action!(move |msg: caro_protocol::MessagePacket| {
                let command_executor = executor_clone.clone();
                let future = async move {
                    println!("{:?}", msg.code());
                    if let caro_protocol::GenericCode::Player(player_code) = msg.code() {
                        command_executor.write().await.execute_request(new_pid, player_code).await;
                    }
                };
                Box::pin(future) as futures::future::BoxFuture<'static, ()>
            }
        )).await;

        player_manager.write().await.handling_request(new_pid).await;
    }
}
