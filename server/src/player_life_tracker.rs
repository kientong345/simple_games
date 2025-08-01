use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::{sync::RwLock, task::JoinHandle};

use crate::{caro_protocol, player_manager};

pub type DisconnectedAction = Arc<tokio::sync::RwLock<dyn FnMut(caro_protocol::PlayerId) -> BoxFuture<'static, ()> + Send + Sync + 'static>>;

pub type TrackingHandler = JoinHandle<()>;

#[macro_export]
macro_rules! make_disconnected_action {
    ($action:expr) => {
        Arc::new(tokio::sync::RwLock::new($action)) as crate::player_life_tracker::DisconnectedAction
    };
}

pub struct PlayerTracker {
    player_manager: Arc<RwLock<player_manager::PlayerContainer>>,
    action_on_disconnect: DisconnectedAction,
    action_on_disconnect_timeout: DisconnectedAction,
}

impl PlayerTracker {
    pub fn new(player_manager: Arc<RwLock<player_manager::PlayerContainer>>) -> Self {
        let action_on_disconnect = make_disconnected_action!(|_pid: caro_protocol::PlayerId| {
            let future = async move {
            };
            Box::pin(future) as BoxFuture<'static, ()>
        });
        let action_on_disconnect_timeout = make_disconnected_action!(|_pid: caro_protocol::PlayerId| {
            let future = async move {
            };
            Box::pin(future) as BoxFuture<'static, ()>
        });
        Self {
            player_manager,
            action_on_disconnect,
            action_on_disconnect_timeout,
        }
    }

    pub fn set_action_on_disconnect(&mut self, action: DisconnectedAction) {
        self.action_on_disconnect = action;
    }

    pub fn set_action_on_disconnect_timeout(&mut self, action: DisconnectedAction) {
        self.action_on_disconnect_timeout = action;
    }

    pub async fn tracking_player(target: Arc<RwLock<PlayerTracker>>) -> TrackingHandler {
        let target_clone = target.clone();
        tokio::spawn(
            async move {
                let target = target_clone.clone();
                loop {
                    let pid = 0;
                    continue; // This is a placeholder for actual player tracking logic
                    target.write().await.player_manager.write().await.set_connection_state(pid, caro_protocol::ConnectState::Disconnected);
                    tokio::spawn(target.read().await.action_on_disconnect.write().await(pid));
                    // continue checking
                    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                    tokio::spawn(target.read().await.action_on_disconnect_timeout.write().await(pid));
                }
            }
        )
    }

    pub async fn stop_tracking_player(handler: Arc<RwLock<TrackingHandler>>) {
        handler.write().await.abort();
    }
}
