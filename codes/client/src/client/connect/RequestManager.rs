extern crate websocket;
use super::super::WebSocket::WebSocket::WebSocket;
use std::thread;
use websocket::sync::Server;
use crate::client::connect::FragmentManager::FragmentManager;
use std::sync::{Arc, Mutex, Condvar};

pub struct RequestManager {
    self_data_port: i32,
    self_ip: String,
    server : websocket::server::WsServer<websocket::server::NoTlsAcceptor, std::net::TcpListener>
}

impl RequestManager {
    pub fn new(port: i32, self_ip: String) -> RequestManager {
        // WebSocket::init(port);
        let mut addr = String::new();
        addr.push_str("127.0.0.1:");
        addr.push_str(&port.to_string()); 
        println!("requestmanager server socket address:{}",addr);
        RequestManager{
            self_data_port:port,
            self_ip:self_ip,
            server:Server::bind(addr).unwrap()
        }
    }

    pub fn run(&mut self,status1:Arc<(Mutex<i32>,Condvar)>) { 
        //TODO:需要锁机制
        println!("WebSocket Server has started on {} :{}.\r\nWaiting for a connection...", self.self_ip, self.self_data_port);
        while(true) {
            let user: WebSocket = WebSocket::new(&mut self.server).unwrap();  //note by lyf:由于websocket解决move时使用了mut，故此处传可变引用
            println!("A user connected.");

            //println!("{}", user);
            let handle = thread::spawn(move || { 
                let mut fragmentManager = FragmentManager::new_user(user);
                fragmentManager.run();
            });
        }
    }
}