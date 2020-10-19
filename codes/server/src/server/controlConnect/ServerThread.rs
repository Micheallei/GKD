use std::net::{TcpListener, TcpStream};
use std::thread;

use super::ClientThread::ClientThread;

pub struct ServerThread {
    server:TcpListener,
}

impl ServerThread{
    pub fn new(addr: String)->ServerThread{
        let listener = TcpListener::bind(addr).unwrap();
        println!("control socket setup!");
        ServerThread{
            server:listener,
        }
    }

    pub fn run(&self){
        loop{
            //println!("before accept");//note:by lyf
            let (socket, addr) = self.server.accept().unwrap();
            println!("accepted a control link!");
            thread::spawn(||{
                let mut client = ClientThread::new(socket);
                client.run();
            });
        }
    }
}