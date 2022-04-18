extern crate websocket;
extern crate libc;

use std::borrow::Borrow;
use libc::{c_char, read, send};
use std::ffi::CStr;
use std::str;
use std::net::TcpStream;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use websocket::client::ClientBuilder;
use websocket::{Message, OwnedMessage};
use websocket::sync::{Client, Reader, Writer};

pub struct Prtc {
    ws_client: Option<Client<TcpStream>>,
    reader: Option<Reader<TcpStream>>,
    writer: Option<Writer<TcpStream>>,
    sender: Option<Sender<OwnedMessage>>,
    receiver: Option<Receiver<OwnedMessage>>
}

impl Prtc {
    fn new() -> Self {
        Prtc {
            ws_client: None,
            reader: None,
            writer: None,
            sender: None,
            receiver: None
        }
    }

    fn connect(&mut self, ip_addr: String, protocol: &str) {
        self.ws_client = Option::from(ClientBuilder::new(&*ip_addr)
            .unwrap()
            .add_protocol(protocol)
            .connect_insecure()
            .unwrap()
        );

        let (
            mut reader,
            mut writer
        ) = self.ws_client.take().unwrap().split().unwrap();
        self.reader = Option::from(reader);
        self.writer = Option::from(writer);

        let (
            sender,
            receiver
        ) = channel();
        self.sender = Option::from(sender);
        self.receiver = Option::from(receiver);
    }

    fn send_message(&mut self, msg: &str) {
        let ws_sender = self.sender.take();

        let trimmed = msg.trim();

        let message = match trimmed {
            "/close" => {
                let _ = ws_sender.as_ref().unwrap().send(OwnedMessage::Close(None));
                return;
            }
            "/ping" => OwnedMessage::Ping(b"PING".to_vec()),
            _ => OwnedMessage::Text(trimmed.to_string()),
        };

        match ws_sender.as_ref().unwrap().send(message) {
            Ok(()) => (),
            Err(e) => {
                println!("Main Loop: {:?}", e);
                return;
            }
        }

        self.sender = ws_sender
    }
}

fn main() {
    let p = Prtc::new();
}

