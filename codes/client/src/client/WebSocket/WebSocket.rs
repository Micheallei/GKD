extern crate websocket;

use std::thread;
use std::sync::mpsc::channel;
use std::io::stdin;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use websocket::{Message, OwnedMessage};
use websocket::client::ClientBuilder;

const CONNECTION: &'static str = "ws://127.0.0.1:2794"; // 需要修改
static mut sta_port:i32 = -1;

pub struct WebSocket{
    //server:websocket::server::WsServer<websocket::server::NoTlsAcceptor, std::net::TcpListener>,
    client: websocket::sync::Client<std::net::TcpStream>
}

impl WebSocket{
    // pub fn init(&mut self,port:i32){
    //     unsafe{sta_port = port;}
    //     let mut addr = String::new();
    //     addr.push_str("127.0.0.1");
    //     addr.push_str(&port.to_string());   
    //     self.server = self.server.bind(addr).unwrap();
    // }

    pub fn new(server: &mut websocket::server::WsServer<websocket::server::NoTlsAcceptor, std::net::TcpListener>) -> WebSocket{
        let request = &mut server.filter_map(Result::ok).next().unwrap();
        // 此处filter_map()返回值是一个迭代器，使用next()方法获得其中一个元素
        //thread::spawn(move || {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                //return;
                // TODO: 接到的连接不是websocket协议时，输出错误信息到log
            }
            //return;
        //});
   
        let client = request.use_protocol("rust-websocket").accept().unwrap();
        WebSocket{
            client:client
        }   
    }

    pub fn sendFile(&mut self, f_path:&PathBuf) {
        let mut f:File = File::open(&f_path.as_path()).unwrap();
        let mut contents = Vec::new();
        f.read_to_end(&mut contents);
        let message = OwnedMessage::Binary(contents);
        &mut self.client.send_message(&message).unwrap();
    }

    pub fn sendMessage(&mut self, msg: String) {
        let message = OwnedMessage::Text(msg);
		& mut self.client.send_message(&message).unwrap();
    }

    pub fn echo(&mut self) -> OwnedMessage{
        let message: OwnedMessage = self.recv();
        println!("Receive Loop: {:?}", message);
        & mut self.client.send_message(&message).unwrap();
        return message;
    }

    pub fn recvFile(&mut self, f_path:&PathBuf) {
        //let mut f:File = File::open(&f_path.as_path()).unwrap();
        let message: OwnedMessage = self.recv();
        match message{
            OwnedMessage::Binary(contents) => {
                fs::write(f_path.as_path(), contents);
            }
            _ => println!("no binary for file\n"),
        }
    }

    pub fn recv(&self) -> OwnedMessage {
        let message_record: OwnedMessage = OwnedMessage::Close(None);
        //let receive_loop = thread::spawn(move || {
            // Receive loop
            let (mut receiver, mut sender) = self.client.split().unwrap();
            let (tx, rx) = channel();
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(m) => m,
                    Err(e) => {
                        println!("Receive Loop: {:?}", e);
                        let _ = tx.send(OwnedMessage::Close(None));
                        return message_record;
                    }
                };
                let message_record = message.clone();
                match message {
                    OwnedMessage::Close(_) => {
                        // Got a close message, so send a close message and return
                        let _ = tx.send(OwnedMessage::Close(None));
                        //return;
                    }
                    OwnedMessage::Ping(data) => {
                        match tx.send(OwnedMessage::Pong(data)) {
                            // Send a pong in response
                            Ok(()) => (),
                            Err(e) => {
                                println!("Receive Loop: {:?}", e);
                                //return;
                            }
                        }
                    }
                    // Say what we received
                    _ => println!("Receive Loop: {:?}", message),
                }
            }
        //});
        return message_record;
    }

}