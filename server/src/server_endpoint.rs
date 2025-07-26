use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use futures::future::BoxFuture;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::caro_protocol::{self, ToMessagePacket};

pub type HandleAction = Arc<tokio::sync::RwLock<dyn FnMut(caro_protocol::MessagePacket) -> BoxFuture<'static, ()> + Send + Sync + 'static>>;

pub type ResponseHandler = JoinHandle<()>;

#[macro_export]
macro_rules! make_action {
    ($action:expr) => {
        Arc::new(tokio::sync::RwLock::new($action)) as crate::server_endpoint::HandleAction
    };
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

    pub async fn accept(&mut self) -> (Receiver, Sender) {
        let (stream, _addr) = self.listener.accept().await.unwrap();
        let (receiver, sender) = stream.into_split();
        (
            Receiver {
                receiver,
                buffer: [0; 1024],
            },
            Sender {sender}
        )
    }
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

pub struct Responser {
    sender: Sender,
}

impl Responser {
    pub fn new(sender: Sender) -> Self {
        Self {
            sender,
        }
    }

    pub async fn send_response(&mut self, message: caro_protocol::MessagePacket) {
        self.sender.send(message.to_serial()).await;
    }
}

pub struct RequestGetter {
    receiver: Receiver,
    action: HandleAction,
}

impl RequestGetter {
    pub fn new(receiver: Receiver) -> Self {
        let action = make_action!(|_msg: caro_protocol::MessagePacket| {
            let future = async move {
            };
            Box::pin(future) as BoxFuture<'static, ()>
        });
        Self {
            receiver,
            action,
        }
    }

    pub fn set_action_on_request(&mut self, action: HandleAction) {
        self.action = action;
    }

    pub fn get_action_on_request(&self) -> HandleAction {
        self.action.clone()
    }

    pub async fn handling_request(target: Arc<RwLock<RequestGetter>>) -> ResponseHandler {
        let target_clone = target.clone();
        tokio::spawn(
            async move {
                let target = target_clone.clone();
                loop {
                    let (msg, bytesread) = target.write().await.receiver.receive().await;
                    if bytesread == 0 {
                        break;
                    }
                    println!("recv {:?}", msg);
                    let msg = msg.to_message_packet();
                    tokio::spawn(target.read().await.action.write().await(msg));
                }
            }
        )
    }

    pub async fn stop_handling_request(handler: Arc<RwLock<ResponseHandler>>) {
        handler.write().await.abort();
    }

}
