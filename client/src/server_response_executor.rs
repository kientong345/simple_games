// most logic would be moved to here

use std::sync::Arc;

use tokio::sync::{Mutex, RwLock};

use crate::{caro_protocol, global_state, output_to_user};

pub struct ResponseExecutor {
    global_state: Arc<RwLock<global_state::GolbalState>>,
    screen_manager: Arc<Mutex<output_to_user::ScreenManager>>,
}

impl ResponseExecutor {
    pub fn new(global_state: Arc<RwLock<global_state::GolbalState>>, screen_manager: Arc<Mutex<output_to_user::ScreenManager>>) -> Self {
        Self {
            global_state,
            screen_manager,
        }
    }

    pub async fn execute_response(&mut self, code: caro_protocol::ServerCode) {

    }
}