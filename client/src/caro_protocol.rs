use serde::{Serialize, Deserialize};

pub const SERVER_ADDRESS: &'static str = "127.0.0.1:12225";

pub type Latitude = i64;
pub type Longtitude = i64;

pub type RoomId = i32;
pub type PlayerId = i32;
pub type GameId = i32;
pub type Coordinate = (Latitude, Longtitude);
pub type Row = Vec<TileState>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameRule {
    TicTacToe,
    FourBlockOne,
    FiveBlockTwo,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TileState {
    Empty,
    Player1,
    Player2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameState {
    Player1Turn,
    Player2Turn,
    Player1Won,
    Player2Won,
    Drew,
    NotInprogress,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConnectState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerState {
    Logged(ConnectState),
    InRoom(ConnectState),
    InGame(ConnectState),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerOrder {
    Player1,
    Player2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameContext {
    pub board_height: usize,
    pub board_width: usize,
    pub player1_move_history: Vec<Coordinate>,
    pub player2_move_history: Vec<Coordinate>,
    pub player1_undone_moves: Vec<Coordinate>,
    pub player2_undone_moves: Vec<Coordinate>,
    pub game_state: GameState,
    pub player1_connection_state: ConnectState,
    pub player2_connection_state: ConnectState,
    pub receiver_order: PlayerOrder,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GeneralRequest {
    PlayerRequestState,
    PlayerExitApplication,
    // response to check alive
    IAmAlive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoggedRequest {
    RequestRoomAsPlayer1(GameRule),
    JoinRoom(RoomId),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InRoomRequest {
    PlayerLeaveRoom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InGameRequest {
    PlayerMove(Coordinate),
    PlayerUndo,
    PlayerRedo,
    PlayerRequestContext,
    PlayerLeaveRoom,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerCode {
    General(GeneralRequest),
    Logged(LoggedRequest),
    InRoom(InRoomRequest),
    InGame(InGameRequest),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum GeneralResponse {
    State(PlayerState),
    // check alive
    AreYouAlive,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoggedResponse {
    JoinedRoomAsPlayer1(RoomId),
    JoinedRoomAsPlayer2(RoomId),
    FailedToCreateRoom,
    FailedToJoinRoom(RoomId),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum InRoomResponse {
    YourRoomIsFull(RoomId),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InGameResponse {
    MoveSuccess,
    MoveUnsuccess,
    Context(GameContext),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerCode {
    General(GeneralResponse),
    Logged(LoggedResponse),
    InRoom(InRoomResponse),
    InGame(InGameResponse),
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