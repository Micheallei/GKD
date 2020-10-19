use std::net::{TcpListener, TcpStream};
use std::thread;

use super::ClientThread::ClientThread;

pub struct ServerThread{
    server:TcpListener,
}

impl ServerThread{

    pub fn new(addr: String)->ServerThread{
        let listener = TcpListener::bind(addr).unwrap();
        println!("data socket setup!");
        ServerThread{
            server:listener,
        }
    }

    pub fn run(&self){
        loop {
            let (stream, addr) = self.server.accept().unwrap();
            println!("accepted a data link!");
            thread::spawn(||{
                let client = ClientThread::new(stream);
                client.run();
            });
        }
    }
}