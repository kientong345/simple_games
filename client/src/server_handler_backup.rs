use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use futures::future::BoxFuture;

use crate::caro_protocol::{self, ToMessagePacket};

pub type HandleAction = Arc<tokio::sync::Mutex<dyn FnMut(caro_protocol::MessagePacket) -> BoxFuture<'static, ()> + Send + 'static>>;

#[macro_export]
macro_rules! make_action {
    ($action:expr) => {
        Arc::new(tokio::sync::Mutex::new($action)) as crate::server_handler::HandleAction
    };
}

pub struct Stream {
    sender: OwnedWriteHalf,
    receiver: OwnedReadHalf,
    buffer: [u8; 1024],
}

impl Stream {
    fn new(stream: TcpStream) -> Self {
        let (receiver, sender) = stream.into_split();
        Self {
            sender,
            receiver,
            buffer: [0; 1024],
        }
    }

    async fn receive(&mut self) -> (Vec<u8>, usize) {
        // let bytesread = loop {
        //     let br = self.stream.read(&mut self.buffer).await.unwrap();
        //     if br !=0 {
        //         break br;
        //     }
        // };
        let bytesread = self.receiver.read(&mut self.buffer).await.unwrap();
        (self.buffer[..bytesread].to_vec(), bytesread)
    }

    async fn send(&mut self, message: Vec<u8>) {
        self.sender.write_all(&message).await.unwrap();
        self.sender.flush().await.unwrap();
    }
}

pub async fn connect_to(dest: &str) -> Stream {
    let stream = TcpStream::connect(dest).await.unwrap();
    Stream::new(stream)
}

pub struct ServerHandler {
    stream: Arc<Mutex<Stream>>,
    action: HandleAction,
}

impl ServerHandler {
    pub fn new(stream: Stream) -> Self {
        let action = make_action!(|_msg: caro_protocol::MessagePacket| {
            let future = async move {
            };
            Box::pin(future) as BoxFuture<'static, ()>
        });
        Self {
            stream: Arc::new(Mutex::new(stream)),
            action,
        }
    }

    pub async fn handling_response(target: Arc<Mutex<ServerHandler>>) {
        loop {
            let target_clone = target.clone();
            let (msg, bytesread) = target_clone.lock().await.stream.lock().await.receive().await;
            if bytesread == 0 {
                break;
            }
            let msg = msg.to_message_packet();
            tokio::spawn(target_clone.lock().await.action.lock().await(msg));
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    }
    
    // pub async fn handling_response(&self) {
    //     let stream = self.stream.clone();
    //     loop {
    //         let (msg, bytesread) = stream.lock().await.receive().await;
    //         if bytesread == 0 {
    //             break;
    //         }
    //         let msg = msg.to_message_packet();
    //         tokio::spawn(self.action.lock().await(msg));
    //         tokio::time::sleep(std::time::Duration::from_millis(5)).await;
    //     }
    // }

    pub async fn handling_response_sync(&self) {
        let (msg, bytesread) = self.stream.lock().await.receive().await;
        if bytesread == 0 {
            return;
        }
        println!("recv {:?}", msg);
        let msg = msg.to_message_packet();
        println!("done");
        tokio::spawn(self.action.lock().await(msg));
    }

    pub async fn set_action_on_response(&mut self, action: HandleAction) {
        self.action = action;
    }

    pub async fn get_action_on_response(&self) -> HandleAction {
        self.action.clone()
    }

    pub async fn send_request(&mut self, message: caro_protocol::MessagePacket) {
        self.stream.lock().await.send(message.to_serial()).await;
    }

    pub fn check_alive(&self) -> bool {
        // Here you would typically check if the stream is still open
        // For example:
        // self.stream.peek(&mut [0; 1]).is_ok()
        true // Placeholder for actual implementation
    }
}
