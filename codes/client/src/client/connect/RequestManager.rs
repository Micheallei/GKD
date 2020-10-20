use super::super::WebSocket::WebSocket::WebSocket;
use super::RequestManager::RequestManager;

pub struct RequestManager {
    self_data_port: i32,
    self_ip: String,
}

impl RequestManager {
    pub fn new(&self, port: i32, self_ip: String) -> RequestManager {
        WebSocke::init(port);
        self.self_data_port = port;
        self.self_ip = self_ip;
    }

    pub fn run() {
        println!("WebSocket Server has started on {} :{}.\r\nWaiting for a connection...", self_ip, self_data_port);
        while(true) {
            let user: WebSocket = WebSocket::new();
            println!("A user connected.");

            println!("{}", user);
            let handle = thread::spawn(move || { 
                let mut fragmentManager = FragmentManager::new_user(user);
                FragmentManager.run();
            });
        }
    }
}