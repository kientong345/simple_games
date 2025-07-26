use std::sync::Arc;

use caro_client::{caro_protocol, client_endpoint::{self, Requester, ResponseGetter}, global_state, input_from_user, make_input_action, make_response_action, output_to_user, server_response_executor, user_command_executor};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let global_state = Arc::new(RwLock::new(global_state::GolbalState::new()));

    let (receiver, sender) = client_endpoint::connect_to(caro_protocol::SERVER_ADDRESS).await;

    let requester = Arc::new(RwLock::new(Requester::new(sender)));
    let response_getter = Arc::new(RwLock::new(ResponseGetter::new(receiver)));

    global_state.write().await.set_connection_state(caro_protocol::ConnectState::Connected);

    let screen_manager = Arc::new(RwLock::new(output_to_user::ScreenManager::new(global_state.clone())));

    let response_executor = Arc::new(RwLock::new(server_response_executor::ResponseExecutor::new(global_state.clone(), screen_manager.clone(), requester.clone())));
    let command_executor = Arc::new(RwLock::new(user_command_executor::CommandExecutor::new(global_state.clone(), screen_manager.clone(), requester.clone())));

    screen_manager.write().await.clean();
    screen_manager.write().await.update().await;
    screen_manager.write().await.enable_prompt_mode().await;

    let response_executor_clone = response_executor.clone();
    response_getter.write().await.set_action_on_response(make_response_action!(move |msg: caro_protocol::MessagePacket| {
        // println!("recv {:?}", msg);
        let response_executor = response_executor_clone.clone();
        let future = async move {
            if let caro_protocol::GenericCode::Server(code) = msg.code() {
                response_executor.write().await.execute_response(code).await;
            }
        };
        Box::pin(future) as futures::future::BoxFuture<'static, ()>
    }));

    ResponseGetter::handling_response(response_getter).await;

    let input_reader = input_from_user::get_input_reader();
    let command_getter = Arc::new(RwLock::new(input_from_user::CommandGetter::new(input_reader)));

    let command_executor_clone = command_executor.clone();
    command_getter.write().await.set_action_on_input(make_input_action!(move |cmd: input_from_user::UserCommand| {
        let command_executor = command_executor_clone.clone();
        let future = async move {
            command_executor.write().await.execute_command(cmd).await;
        };
        Box::pin(future) as futures::future::BoxFuture<'static, ()>
    }));

    input_from_user::CommandGetter::handling_input(command_getter).await;

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }

}
