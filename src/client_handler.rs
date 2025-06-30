use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use futures::future::BoxFuture;

use crate::caro_protocol::{self, ToMessagePacket};

pub type HandleAction = Arc<tokio::sync::Mutex<dyn FnMut(caro_protocol::MessagePacket) -> BoxFuture<'static, ()> + Send + 'static>>;

#[macro_export]
macro_rules! make_action {
    ($action:expr) => {
        Arc::new(tokio::sync::Mutex::new($action)) as crate::client_handler::HandleAction
    };
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
        // let bytesread = loop {
        //     let br = self.stream.read(&mut self.buffer).await.unwrap();
        //     if br !=0 {
        //         break br;
        //     }
        // };
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

    pub async fn handling_request(target: Arc<Mutex<ClientHandler>>) {
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

    pub async fn set_action_on_request(&mut self, action: HandleAction) {
        self.action = action;
    }

    pub async fn get_action_on_request(&self) -> HandleAction {
        self.action.clone()
    }

    pub async fn response(&mut self, message: caro_protocol::MessagePacket) {
        self.stream.lock().await.send(message.to_serial()).await;
    }

    pub fn check_alive(&self) -> bool {
        // Here you would typically check if the stream is still open
        // For example:
        // self.stream.peek(&mut [0; 1]).is_ok()
        true // Placeholder for actual implementation
    }
}