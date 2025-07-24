// most logic would be moved to here

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::global_state;

struct ResponseExecutor {
    global_state: Arc<RwLock<global_state::GolbalState>>,
}

impl ResponseExecutor {
    pub fn new(global_state: Arc<RwLock<global_state::GolbalState>>) -> Self {
        Self {
            global_state,
        }
    }
}