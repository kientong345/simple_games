use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use futures::future::BoxFuture;


pub type HandleAction = Arc<tokio::sync::Mutex<dyn FnMut(MessagePacket) -> BoxFuture<'static, ()> + Send + 'static>>;

#[macro_export]
macro_rules! make_action {
    ($action:expr) => {
        Arc::new(tokio::sync::Mutex::new($action)) as crate::communication::HandleAction
    };
}

pub const SERVER_ADDRESS: &'static str = "127.0.0.1:12225";

#[derive(Debug, Clone, Copy)]
pub enum GameRule {
    TicTacToe,
    FourBlockOne,
    FiveBlockTwo,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerCommand {
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

pub enum ServerResponse {

}

#[derive(Debug, Clone)]
pub struct MessagePacket {
    raw_data: Vec<u8>,
}

impl<'a> MessagePacket {
    pub fn command(&self) -> PlayerCommand {
        todo!()
    }

    pub fn to_serial(self) -> Vec<u8> {
        self.raw_data
    }
}

pub trait ToMessagePacket {
    fn to_message_packet(self) -> MessagePacket;
}

impl ToMessagePacket for &[u8] {
    fn to_message_packet(self) -> MessagePacket {
        MessagePacket {
            raw_data: self.to_vec(),
        }
    }
}

pub struct Stream {
    stream: TcpStream,
    buffer: [u8; 1024],
}

impl Stream {
    fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: [0; 1024],
        }
    }

    async fn receive(&mut self) -> (Vec<u8>, usize) {
        let bytesread = self.stream.read(&mut self.buffer).await.unwrap();
        (self.buffer[..bytesread].to_vec(), bytesread)
    }

    async fn send(&mut self, message: Vec<u8>) {
        self.stream.write_all(&message).await.unwrap();
        self.stream.flush().await.unwrap();
    }
}

pub struct Listener {
    listener: TcpListener,
}

impl Listener {
    pub async fn new(addr: &str) -> Self {
        Self {
            listener: TcpListener::bind(addr).await.unwrap(),
        }
    }

    pub async fn accept(&mut self) -> Stream {
        let (stream, _addr) = self.listener.accept().await.unwrap();
        Stream::new(stream)
    }
}

pub struct ClientHandler {
    stream: Arc<Mutex<Stream>>,
    action: HandleAction,
}

impl ClientHandler {
    pub fn new(stream: Stream) -> Self {
        let action = make_action!(|_msg: MessagePacket| {
            let future = async move {
            };
            Box::pin(future) as BoxFuture<'static, ()>
        });
        Self {
            stream: Arc::new(Mutex::new(stream)),
            action,
        }
    }

    pub async fn handling_request(target: Arc<Mutex<ClientHandler>>) {
        loop {
            let target_clone = target.clone();
            let (msg, bytesread) = target_clone.lock().await.stream.lock().await.receive().await;
            let msg = msg.to_message_packet();
            if bytesread == 0 {
                break;
            }
            tokio::spawn(target_clone.lock().await.action.lock().await(msg));
        }
    }

    pub async fn set_action_on_request(&mut self, action: HandleAction) {
        self.action = action;
    }

    pub async fn get_action_on_request(&self) -> HandleAction {
        self.action.clone()
    }

    pub async fn response(&mut self, message: MessagePacket) {
        self.stream.lock().await.send(message.to_serial()).await;
    }

    pub fn check_alive(&self) -> bool {
        // Here you would typically check if the stream is still open
        // For example:
        // self.stream.peek(&mut [0; 1]).is_ok()
        true // Placeholder for actual implementation
    }
}