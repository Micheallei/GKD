extern crate websocket;
use super::super::WebSocket::WebSocket::WebSocket;
use std::thread;
use websocket::sync::Server;
use crate::client::connect::FragmentManager::FragmentManager;

pub struct RequestManager {
    self_data_port: i32,
    self_ip: String,
    server : websocket::server::WsServer<websocket::server::NoTlsAcceptor, std::net::TcpListener>
}

impl RequestManager {
    pub fn new(port: i32, self_ip: String) -> RequestManager {
        // WebSocket::init(port);
        let mut addr = String::new();
        addr.push_str("127.0.0.1");
        addr.push_str(&port.to_string()); 
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
            let user: WebSocket = WebSocket::new(&self.server);
            println!("A user connected.");

            //println!("{}", user);
            let handle = thread::spawn(move || { 
                let mut fragmentManager = FragmentManager::new_user(user);
                fragmentManager.run();
            });
        }
    }
}