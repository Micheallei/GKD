extern crate websocket;

use std::thread;
use std::sync::mpsc::channel;
use std::io::stdin;

use websocket::{Message, OwnedMessage};
use websocket::client::ClientBuilder;

const CONNECTION: &'static str = "ws://127.0.0.1:2794"; // 需要修改
static mut sta_port:i32 = -1;

struct WebSocket{
    server:websocket::server::sync::Server,
    client:// 类型?
}

impl WebSocket{
    pub init(&mut self,port:i32){
        unsafe{sta_port = port;}
        let mut addr = String::new();
        addr.push("127.0.0.1");
        addr.push_str(port.to_string());   
        self.server = Server::bind(addr).unwrap();
    }

    pub new(&mut self) -> WebSocket{
        for request in self.server.filter_map(Result::ok) {
            // 需要把循环改了
            // // Spawn a new thread for each connection.
            // thread::spawn(move || {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
                request.reject().unwrap();
                return WebSocket{server:None,client:None};
            }
            

            self.client = request.use_protocol("rust-websocket").accept().unwrap();
            
    }

    pub sendFile(f_path:&PathBuf) {
        let mut f:File = File::open(&f_path.as_path()).unwrap();
        sendBin()
    }
}