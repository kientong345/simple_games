
use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

use crate::{caro_protocol, global_state, input_from_user, output_to_user};

pub struct CommandExecutor {
    global_state: Arc<RwLock<global_state::GolbalState>>,
    screen_manager: Arc<Mutex<output_to_user::ScreenManager>>,
}

impl CommandExecutor {
    pub fn new(global_state: Arc<RwLock<global_state::GolbalState>>, screen_manager: Arc<Mutex<output_to_user::ScreenManager>>) -> Self {
        Self {
            global_state,
            screen_manager,
        }
    }

    pub async fn execute_command(&mut self, command: input_from_user::UserCommand) {

    }
}