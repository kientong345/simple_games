
#[derive(Debug, Clone, Copy)]
pub enum GameRule {
    TicTacToe,
    FourBlockOne,
    FiveBlockTwo,
}

#[derive(Debug, Clone, Copy)]
pub enum TileState {
    Empty,
    Player1,
    Player2,
}

#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Player1Turn,
    Player2Turn,
    Player1Won,
    Player2Won,
    Drew,
    NotInprogress,
}

#[derive(Debug, Clone, Copy)]
pub enum ConnectState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerState {
    Logged(ConnectState),
    Waiting(ConnectState),
    InGame(ConnectState),
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum GenericCode {
    Player(PlayerCode),
    Server(ServerCode),
}

#[derive(Debug, Clone)]
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
        // self.code
        todo!()
    }
}

pub trait ToMessagePacket {
    fn to_message_packet(self) -> MessagePacket;
}

impl ToMessagePacket for &[u8] {
    fn to_message_packet(self) -> MessagePacket {
        // MessagePacket {
        //     raw_data: self.to_vec(),
        // }
        todo!()
    }
}