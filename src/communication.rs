use std::sync::{Arc, Mutex};

type Callback = Arc<Mutex<dyn FnMut(MessagePacket)>>;

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

pub struct MessagePacket {

}

impl<'a> MessagePacket {
    pub fn command(&self) -> PlayerCommand {
        todo!()
    }

    pub fn to_serial(self) -> &'a [u8] {
        todo!()
    }
}

pub trait ToMessagePacket {
    fn to_message_packet(self) -> MessagePacket;
}

impl ToMessagePacket for &[u8] {
    fn to_message_packet(self) -> MessagePacket {
        todo!()
    }
}

pub struct Stream {

}

pub struct Listener {
    
}

impl Listener {
    pub fn new(addr: &str) -> Self {
        todo!()
    }
    pub fn accept(&self) -> Result<Stream, &str> {
        todo!()
    }
}

pub struct Communicator {
    stream: Arc<Mutex<Stream>>,
    callback: Arc<Mutex<Option<Callback>>>,
}

impl Communicator {
    pub fn new(stream: Stream) -> Arc<Mutex<Self>> {
        let itself = Arc::new(Mutex::new(Self {
            stream: Arc::new(Mutex::new(stream)),
            callback: Arc::new(Mutex::new(None)),
        }));

        // Self::spawn_handler(itself.clone());

        itself
    }

    pub fn spawn_handler(handler: Arc<Self>) {
        // tokio::task::spawn(async move {
        //     let mut buf = [0u8; 1024];

        //     loop {
        //         let n = {
        //             let mut stream_guard = handler.stream.lock().unwrap();
        //             match stream_guard.read(&mut buf).await {
        //                 Ok(0) => {
        //                     println!("Connection closed");
        //                     break;
        //                 }
        //                 Ok(n) => n,
        //                 Err(e) => {
        //                     eprintln!("Read error: {:?}", e);
        //                     break;
        //                 }
        //             }
        //         };

        //         let data = buf[..n].to_message_packet();

        //         // Call the callback if present
        //         if let Some(cb) = &*handler.callback.lock().unwrap() {
        //             (cb.lock().unwrap())(data);
        //         }
        //     }
        // });
    }

    pub fn set_action_on_request<F>(&mut self, action: F)
    where
        F: FnMut(MessagePacket) + 'static,
    {
        let mut callback = self.callback.lock().unwrap();
        *callback = Some(Arc::new(Mutex::new(action)));
    }

    pub fn get_action_on_request(&self) -> Option<Callback> {
        let callback = self.callback.lock().unwrap();
        callback.clone()
    }

    pub fn response(&self, message: MessagePacket) {
        // Here you would typically write the message back to the stream
        // For example:
        // self.stream.write_all(&message.to_bytes()).unwrap();
    }

    pub fn check_alive(&self) -> bool {
        // Here you would typically check if the stream is still open
        // For example:
        // self.stream.peek(&mut [0; 1]).is_ok()
        true // Placeholder for actual implementation
    }
}