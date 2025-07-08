use std::sync::Arc;

use caro_client::{caro_protocol::{self, MessagePacket}, client_state, make_action, server_handler::{self, Requester, ResponseGetter}, user_handler};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let global_state = Arc::new(Mutex::new(client_state::ClientState::new()));

    let (receiver, sender) = server_handler::connect_to(caro_protocol::SERVER_ADDRESS).await;

    let requester = Arc::new(Mutex::new(Requester::new(sender)));
    let response_getter = Arc::new(Mutex::new(ResponseGetter::new(receiver)));

    global_state.lock().await.set_connection_state(caro_protocol::ConnectState::Connected);

    response_getter.lock().await.set_action_on_response(make_action!(move |msg: caro_protocol::MessagePacket| {
        // println!("recv {:?}", msg);
        let global_state_clone = global_state.clone();
        let future = async move {
            let global_state = global_state_clone.clone();
            match msg.code() {
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::JoinedRoomAsPlayer1(rid)) => {
                    let conn_state = global_state.lock().await.get_connection_state();
                    match conn_state {
                        caro_protocol::ConnectState::Connected => {
                            global_state.lock().await.set_player_state(caro_protocol::PlayerState::Waiting(caro_protocol::ConnectState::Connected));
                            global_state.lock().await.set_rid(rid);
                            user_handler::print_notification("JoinedRoomAsPlayer1 in room:");
                            user_handler::print_notification(&rid.to_string());
                        },
                        caro_protocol::ConnectState::Disconnected => {
                        },
                    }
                },
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::JoinedRoomAsPlayer2(rid)) => {
                    let conn_state = global_state.lock().await.get_connection_state();
                    match conn_state {
                        caro_protocol::ConnectState::Connected => {
                            global_state.lock().await.set_player_state(caro_protocol::PlayerState::Waiting(caro_protocol::ConnectState::Connected));
                            global_state.lock().await.set_rid(rid);
                            user_handler::print_notification("JoinedRoomAsPlayer2 in room:");
                            user_handler::print_notification(&rid.to_string());
                        },
                        caro_protocol::ConnectState::Disconnected => {

                        },
                    }
                },
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::FailedToCreateRoom) => {

                },
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::FailedToJoinRoom(rid)) => {

                },
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::YourRoomIsFull(rid)) => {
                    let conn_state = global_state.lock().await.get_connection_state();
                    match conn_state {
                        caro_protocol::ConnectState::Connected => {
                            global_state.lock().await.set_player_state(caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected));
                            global_state.lock().await.set_rid(rid);
                            user_handler::print_notification("game is ready!");
                        },
                        caro_protocol::ConnectState::Disconnected => {

                        },
                    }
                },
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::MoveSuccess) => {

                },
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::MoveUnsuccess) => {

                },
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::Context(game_context)) => {
                    let caro_board = game_context.board;
                    user_handler::print_caro_board(caro_board);
                },
                _ => (),
            }
        };
        Box::pin(future) as futures::future::BoxFuture<'static, ()>
    }));

    ResponseGetter::handling_response(response_getter).await;

    loop {
        let code = user_handler::get_command();
        let new_packet = MessagePacket::new_player_packet(code);
        println!("send: {:?}", new_packet);
        requester.lock().await.send_request(new_packet).await;
    }

}
