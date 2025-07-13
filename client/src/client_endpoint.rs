use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use futures::future::BoxFuture;
use tokio::task::JoinHandle;

use crate::caro_protocol::{self, ToMessagePacket};

pub type HandleAction = Arc<tokio::sync::Mutex<dyn FnMut(caro_protocol::MessagePacket) -> BoxFuture<'static, ()> + Send + 'static>>;

pub type ResponseHandler = JoinHandle<()>;

#[macro_export]
macro_rules! make_response_action {
    ($action:expr) => {
        Arc::new(tokio::sync::Mutex::new($action)) as crate::client_endpoint::HandleAction
    };
}

pub struct Sender {
    sender: OwnedWriteHalf,
}

impl Sender {
    async fn send(&mut self, message: Vec<u8>) {
        self.sender.write_all(&message).await.unwrap();
        self.sender.flush().await.unwrap();
    }
}

pub struct Receiver {
    receiver: OwnedReadHalf,
    buffer: [u8; 1024],
}

impl Receiver {
    async fn receive(&mut self) -> (Vec<u8>, usize) {
        let bytesread = self.receiver.read(&mut self.buffer).await.unwrap();
        (self.buffer[..bytesread].to_vec(), bytesread)
    }
}

pub async fn connect_to(dest: &str) -> (Receiver, Sender) {
    let (receiver, sender) = TcpStream::connect(dest).await.unwrap().into_split();
    (
        Receiver {
            receiver,
            buffer: [0; 1024],
        },
        Sender {sender}
    )
}

pub struct Requester {
    sender: Sender,
}

impl Requester {
    pub fn new(sender: Sender) -> Self {
        Self {
            sender,
        }
    }

    pub async fn send_request(&mut self, message: caro_protocol::MessagePacket) {
        self.sender.send(message.to_serial()).await;
    }
}

pub struct ResponseGetter {
    receiver: Receiver,
    action: HandleAction,
}

impl ResponseGetter {
    pub fn new(receiver: Receiver) -> Self {
        let action = make_response_action!(|_msg: caro_protocol::MessagePacket| {
            let future = async move {
            };
            Box::pin(future) as BoxFuture<'static, ()>
        });
        Self {
            receiver,
            action,
        }
    }

    pub fn set_action_on_response(&mut self, action: HandleAction) {
        self.action = action;
    }

    pub fn get_action_on_response(&self) -> HandleAction {
        self.action.clone()
    }

    pub async fn handling_response(target: Arc<Mutex<ResponseGetter>>) -> ResponseHandler {
        let target_clone = target.clone();
        tokio::spawn(
            async move {
                let target = target_clone.clone();
                loop {
                    let (msg, bytesread) = target.lock().await.receiver.receive().await;
                    if bytesread == 0 {
                        break;
                    }
                    let msg = msg.to_message_packet();
                    tokio::spawn(target.lock().await.action.lock().await(msg));
                }
            }
        )
    }

    pub fn stop_handling_response(handler: ResponseHandler) {
        handler.abort();
    }

}
