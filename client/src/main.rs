use std::sync::Arc;

use caro_client::{caro_protocol::{self, MessagePacket}, client_endpoint::{self, Requester, ResponseGetter}, global_state, input_from_user, output_to_user, make_input_action, make_response_action};
use tokio::sync::{Mutex, RwLock};

#[tokio::main]
async fn main() {
    let global_state = Arc::new(RwLock::new(global_state::GolbalState::new()));

    let (receiver, sender) = client_endpoint::connect_to(caro_protocol::SERVER_ADDRESS).await;

    let requester = Arc::new(Mutex::new(Requester::new(sender)));
    let response_getter = Arc::new(Mutex::new(ResponseGetter::new(receiver)));

    global_state.write().await.set_connection_state(caro_protocol::ConnectState::Connected);

    let screen_manager = Arc::new(Mutex::new(output_to_user::ScreenManager::new(global_state.clone())));

    screen_manager.lock().await.clean();
    screen_manager.lock().await.update().await;
    screen_manager.lock().await.enable_prompt_mode().await;

    response_getter.lock().await.set_action_on_response(make_response_action!(move |msg: caro_protocol::MessagePacket| {
        // println!("recv {:?}", msg);
        let global_state_clone = global_state.clone();
        let screen_manager_clone = screen_manager.clone();
        let future = async move {
            let global_state = global_state_clone.clone();
            let screen_manager = screen_manager_clone.clone();
            match msg.code() {
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::JoinedRoomAsPlayer1(rid)) => {
                    let conn_state = global_state.read().await.get_connection_state();
                    match conn_state {
                        caro_protocol::ConnectState::Connected => {
                            global_state.write().await.set_player_state(caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected));
                            global_state.write().await.set_current_rid(rid);
                            screen_manager.lock().await.clean();
                            screen_manager.lock().await.set_player_order(caro_protocol::PlayerOrder::Player1);
                            screen_manager.lock().await.update().await;
                            let log_content = "JoinedRoomAsPlayer1 in room".to_string() + &rid.to_string();
                            screen_manager.lock().await.log(log_content).await;
                            screen_manager.lock().await.enable_prompt_mode().await;
                        },
                        caro_protocol::ConnectState::Disconnected => {
                        },
                    }
                },
                caro_protocol::GenericCode::Server(caro_protocol::ServerCode::JoinedRoomAsPlayer2(rid)) => {
                    let conn_state = global_state.read().await.get_connection_state();
                    match conn_state {
                        caro_protocol::ConnectState::Connected => {
                            global_state.write().await.set_player_state(caro_protocol::PlayerState::InRoom(caro_protocol::ConnectState::Connected));
                            global_state.write().await.set_current_rid(rid);
                            screen_manager.lock().await.clean();
                            screen_manager.lock().await.set_player_order(caro_protocol::PlayerOrder::Player2);
                            screen_manager.lock().await.update().await;
                            let log_content = "JoinedRoomAsPlayer2".to_string() + &rid.to_string();
                            screen_manager.lock().await.log(log_content).await;
                            screen_manager.lock().await.enable_prompt_mode().await;
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
                    let conn_state = global_state.read().await.get_connection_state();
                    match conn_state {
                        caro_protocol::ConnectState::Connected => {
                            global_state.write().await.set_player_state(caro_protocol::PlayerState::InGame(caro_protocol::ConnectState::Connected));
                            global_state.write().await.set_current_rid(rid);
                            screen_manager.lock().await.clean();
                            screen_manager.lock().await.update().await;
                            screen_manager.lock().await.log("game is ready!".to_string()).await;
                            screen_manager.lock().await.enable_prompt_mode().await;
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
                    // screen_manager::print_caro_context(game_context);
                    screen_manager.lock().await.update_game_context(&game_context);
                    screen_manager.lock().await.update_board_only().await;
                },
                _ => (),
            }
        };
        Box::pin(future) as futures::future::BoxFuture<'static, ()>
    }));

    ResponseGetter::handling_response(response_getter).await;

    let input_reader = input_from_user::get_input_reader();
    let command_getter = Arc::new(Mutex::new(input_from_user::CommandGetter::new(input_reader)));

    let requester_clone = requester.clone();
    command_getter.lock().await.set_action_on_input(make_input_action!(move |cmd: input_from_user::UserCommand| {
        let requester = requester_clone.clone();
        let future = async move {
            match cmd {
                input_from_user::UserCommand::RequestNewRoom(game_rule) => {
                    match game_rule {
                        caro_protocol::GameRule::TicTacToe => {
                            let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::TicTacToe);
                            let new_packet = MessagePacket::new_player_packet(code);
                            // println!("send: {:?}", new_packet);
                            requester.lock().await.send_request(new_packet).await;
                        },
                        caro_protocol::GameRule::FourBlockOne => {
                            let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::FourBlockOne);
                            let new_packet = MessagePacket::new_player_packet(code);
                            // println!("send: {:?}", new_packet);
                            requester.lock().await.send_request(new_packet).await;
                        },
                        caro_protocol::GameRule::FiveBlockTwo => {
                            let code = caro_protocol::PlayerCode::RequestRoomAsPlayer1(caro_protocol::GameRule::FiveBlockTwo);
                            let new_packet = MessagePacket::new_player_packet(code);
                            // println!("send: {:?}", new_packet);
                            requester.lock().await.send_request(new_packet).await;
                        },
                    }
                },
                input_from_user::UserCommand::JoinRoom(rid) => {
                    let code = caro_protocol::PlayerCode::JoinRoom(rid);
                    let new_packet = MessagePacket::new_player_packet(code);
                    // println!("send: {:?}", new_packet);
                    requester.lock().await.send_request(new_packet).await;
                },
                input_from_user::UserCommand::Move(coor) => {
                    let coor = (coor.0, coor.1);
                    let code = caro_protocol::PlayerCode::PlayerMove(coor);
                    let new_packet = MessagePacket::new_player_packet(code);
                    // println!("send: {:?}", new_packet);
                    requester.lock().await.send_request(new_packet).await;
                },
                _ => {

                }
            }
        };
        Box::pin(future) as futures::future::BoxFuture<'static, ()>
    }));

    input_from_user::CommandGetter::handling_input(command_getter).await;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

}
