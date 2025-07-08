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
macro_rules! make_action {
    ($action:expr) => {
        Arc::new(tokio::sync::Mutex::new($action)) as crate::server_handler::HandleAction
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
                    println!("recv {:?}", msg);
                    let msg = msg.to_message_packet();
                    tokio::spawn(target.lock().await.action.lock().await(msg));
                    tokio::time::sleep(std::time::Duration::from_millis(5)).await;
                }
            }
        )
    }

    pub fn stop_handling_response(handler: ResponseHandler) {
        handler.abort();
    }

}

// pub struct ServerHandler {
//     stream: Arc<Mutex<Stream>>,
//     action: HandleAction,
// }

// impl ServerHandler {
//     pub fn new(stream: Stream) -> Self {
//         let action = make_action!(|_msg: caro_protocol::MessagePacket| {
//             let future = async move {
//             };
//             Box::pin(future) as BoxFuture<'static, ()>
//         });
//         Self {
//             stream: Arc::new(Mutex::new(stream)),
//             action,
//         }
//     }

//     pub async fn handling_response(target: Arc<Mutex<ServerHandler>>) {
//         loop {
//             let target_clone = target.clone();
//             let (msg, bytesread) = target_clone.lock().await.stream.lock().await.receive().await;
//             if bytesread == 0 {
//                 break;
//             }
//             let msg = msg.to_message_packet();
//             tokio::spawn(target_clone.lock().await.action.lock().await(msg));
//             tokio::time::sleep(std::time::Duration::from_millis(5)).await;
//         }
//     }
    
//     // pub async fn handling_response(&self) {
//     //     let stream = self.stream.clone();
//     //     loop {
//     //         let (msg, bytesread) = stream.lock().await.receive().await;
//     //         if bytesread == 0 {
//     //             break;
//     //         }
//     //         let msg = msg.to_message_packet();
//     //         tokio::spawn(self.action.lock().await(msg));
//     //         tokio::time::sleep(std::time::Duration::from_millis(5)).await;
//     //     }
//     // }

//     pub async fn handling_response_sync(&self) {
//         let (msg, bytesread) = self.stream.lock().await.receive().await;
//         if bytesread == 0 {
//             return;
//         }
//         println!("recv {:?}", msg);
//         let msg = msg.to_message_packet();
//         println!("done");
//         tokio::spawn(self.action.lock().await(msg));
//     }

//     pub async fn set_action_on_response(&mut self, action: HandleAction) {
//         self.action = action;
//     }

//     pub async fn get_action_on_response(&self) -> HandleAction {
//         self.action.clone()
//     }

//     pub async fn send_request(&mut self, message: caro_protocol::MessagePacket) {
//         self.stream.lock().await.send(message.to_serial()).await;
//     }

//     pub fn check_alive(&self) -> bool {
//         // Here you would typically check if the stream is still open
//         // For example:
//         // self.stream.peek(&mut [0; 1]).is_ok()
//         true // Placeholder for actual implementation
//     }
// }
