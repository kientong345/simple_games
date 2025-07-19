use std::sync::Arc;

use caro_client::{caro_protocol::{self, MessagePacket}, client_endpoint::{self, Requester, ResponseGetter}, client_state, command_getter, screen_manager, make_input_action, make_response_action};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let global_state = Arc::new(Mutex::new(client_state::ClientState::new()));

    let (receiver, sender) = client_endpoint::connect_to(caro_protocol::SERVER_ADDRESS).await;

    let requester = Arc::new(Mutex::new(Requester::new(sender)));
    let response_getter = Arc::new(Mutex::new(ResponseGetter::new(receiver)));

    global_state.lock().await.set_connection_state(caro_protocol::ConnectState::Connected);

    response_getter.lock().await.set_action_on_response(make_response_action!(move |msg: caro_protocol::MessagePacket| {
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
                            screen_manager::print_notification("JoinedRoomAsPlayer1 in room:");
                            screen_manager::print_notification(&rid.to_string());
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
                            screen_manager::print_notification("JoinedRoomAsPlayer2 in room:");
                            screen_manager::print_notification(&rid.to_string());
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
                            screen_manager::print_notification("game is ready!");
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
                    screen_manager::print_caro_context(game_context);
                },
                _ => (),
            }
        };
        Box::pin(future) as futures::future::BoxFuture<'static, ()>
    }));

    ResponseGetter::handling_response(response_getter).await;

    let input_reader = command_getter::get_input_reader();
    let command_getter = Arc::new(Mutex::new(command_getter::CommandGetter::new(input_reader)));

    let requester_clone = requester.clone();
    command_getter.lock().await.set_action_on_input(make_input_action!(move |cmd: command_getter::UserCommand| {
        let requester = requester_clone.clone();
        let future = async move {
            match cmd {
                command_getter::UserCommand::RequestNewRoom(game_rule) => {
                    match game_rule {
                        command_getter::GameRule::TicTacToe => {
                            let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::TicTacToe);
                            let new_packet = MessagePacket::new_player_packet(code);
                            println!("send: {:?}", new_packet);
                            requester.lock().await.send_request(new_packet).await;
                        },
                        command_getter::GameRule::FourBlockOne => {
                            let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::FourBlockOne);
                            let new_packet = MessagePacket::new_player_packet(code);
                            println!("send: {:?}", new_packet);
                            requester.lock().await.send_request(new_packet).await;
                        },
                        command_getter::GameRule::FiveBlockTwo => {
                            let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::FiveBlockTwo);
                            let new_packet = MessagePacket::new_player_packet(code);
                            println!("send: {:?}", new_packet);
                            requester.lock().await.send_request(new_packet).await;
                        },
                    }
                },
                command_getter::UserCommand::JoinRoom(rid) => {
                    let code = caro_protocol::PlayerCode::JoinRoom(rid);
                    let new_packet = MessagePacket::new_player_packet(code);
                    println!("send: {:?}", new_packet);
                    requester.lock().await.send_request(new_packet).await;
                },
                command_getter::UserCommand::Move(coor) => {
                    let coor = (coor.0, coor.1);
                    let code = caro_protocol::PlayerCode::PlayerMove(coor);
                    let new_packet = MessagePacket::new_player_packet(code);
                    println!("send: {:?}", new_packet);
                    requester.lock().await.send_request(new_packet).await;
                },
                _ => {

                }
            }
        };
        Box::pin(future) as futures::future::BoxFuture<'static, ()>
    }));

    command_getter::CommandGetter::handling_input(command_getter).await;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

}
