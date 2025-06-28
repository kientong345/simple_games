use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GameRule {
    TicTacToe,
    FourBlockOne,
    FiveBlockTwo,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TileState {
    Empty,
    Player1,
    Player2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum GameState {
    Player1Turn,
    Player2Turn,
    Player1Won,
    Player2Won,
    Drew,
    NotInprogress,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ConnectState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerState {
    Logged(ConnectState),
    Waiting(ConnectState),
    InGame(ConnectState),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameContext {
    pub board: Vec<Vec<TileState>>,
    pub player1_move_history: Vec<Coordinate>,
    pub player2_move_history: Vec<Coordinate>,
    pub player1_undone_moves: Vec<Coordinate>,
    pub player2_undone_moves: Vec<Coordinate>,
    pub game_state: GameState,
    pub player1_state: PlayerState,
    pub player2_state: PlayerState,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PlayerCode {
    // pregame
    RequestRoomAsPlayer1(GameRule),
    JoinRoomAsPlayer2(i32),
    // ingame
    Player1Move(i64, i64),
    Player2Move(i64, i64),
    Player1Undo,
    Player2Undo,
    Player1Redo,
    Player2Redo,
    Player1RequestContext,
    Player2RequestContext,
    Player1Leave,
    Player2Leave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerCode {
    // pregame
    JoinedRoomAsPlayer1(i32),
    JoinedRoomAsPlayer2(i32),
    FailedToJoinRoom(i32),
    YourRoomIsFull,
    // ingame
    MoveSuccess,
    MoveUnsuccess,
    Context(GameContext),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GenericCode {
    Player(PlayerCode),
    Server(ServerCode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessagePacket {
    code: GenericCode,
}

impl<'a> MessagePacket {
    pub fn new_server_packet(code: ServerCode) -> Self {
        Self {
            code: GenericCode::Server(code),
        }
    }

    pub fn new_player_packet(code: PlayerCode) -> Self {
        Self {
            code: GenericCode::Player(code),
        }
    }

    pub fn code(&self) -> GenericCode {
        self.code.clone()
    }

    pub fn to_serial(self) -> Vec<u8> {
        let json_str = serde_json::to_string(&self.code).unwrap();
        json_str.as_bytes().to_vec()
    }
}

pub trait ToMessagePacket {
    fn to_message_packet(self) -> MessagePacket;
}

impl ToMessagePacket for &[u8] {
    fn to_message_packet(self) -> MessagePacket {
        let json_str = String::from_utf8_lossy(self);
        let code: GenericCode = serde_json::from_str(&json_str).unwrap();
        match code {
            GenericCode::Server(server_code) => {
                MessagePacket::new_server_packet(server_code)
            },
            GenericCode::Player(player_code) => {
                MessagePacket::new_player_packet(player_code)
            },
        }
    }
}