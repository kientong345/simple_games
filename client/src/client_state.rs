use crate::caro_protocol;


pub struct ClientState {
    player_state: caro_protocol::PlayerState,
    current_rid: i32,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            player_state: caro_protocol::PlayerState::Logged(caro_protocol::ConnectState::Disconnected),
            current_rid: -1,
        }
    }

    pub fn set_player_state(&mut self, player_state: caro_protocol::PlayerState) {
        self.player_state = player_state;
    }

    pub fn get_player_state(&self) -> caro_protocol::PlayerState {
        self.player_state
    }

    pub fn set_connection_state(&mut self, connection_state: caro_protocol::ConnectState) {
        match self.player_state {
            caro_protocol::PlayerState::Logged(_conn_state) => {
                self.player_state = caro_protocol::PlayerState::Logged(connection_state);
            },
            caro_protocol::PlayerState::Waiting(_conn_state) => {
                self.player_state = caro_protocol::PlayerState::Waiting(connection_state);
            },
            caro_protocol::PlayerState::InGame(_conn_state) => {
                self.player_state = caro_protocol::PlayerState::InGame(connection_state);
            },
        }
    }

    pub fn get_connection_state(&self) -> caro_protocol::ConnectState {
        match self.player_state {
            caro_protocol::PlayerState::Logged(conn_state) => {
                conn_state
            },
            caro_protocol::PlayerState::Waiting(conn_state) => {
                conn_state
            },
            caro_protocol::PlayerState::InGame(conn_state) => {
                conn_state
            },
        }
    }

    pub fn set_rid(&mut self, rid: i32) {
        self.current_rid = rid;
    }

    pub fn get_rid(&mut self) -> i32 {
        self.current_rid
    }
}