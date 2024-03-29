use std::net::TcpListener;
use std::thread;
use log::{info,warn,debug,error,trace};
use log4rs;

use super::ClientThread::ClientThread;

pub struct ServerThread {
    server:TcpListener,
}

impl ServerThread{
    pub fn new(addr: String)->ServerThread{
        let listener = TcpListener::bind(addr).unwrap();
        //println!("control socket setup!");
        info!("control socket setup!");
        ServerThread{
            server:listener,
        }
    }

    pub fn run(&self){
        loop{
            let (socket, addr) = self.server.accept().unwrap();
            //println!("accepted a control link!");
            info!("accepted a control link!");
            thread::spawn(||{
                let mut client = ClientThread::new(socket);
                client.run();
            });
        }
    }
}