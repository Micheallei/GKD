use std::net::{TcpStream, Ipv4Addr, Shutdown};
use std::string::String;
use std::io::BufReader;
use std::io::prelude::*;
use std::{thread, time};
use std::sync::{Arc, Mutex, Condvar};
use std::convert::TryInto;
use log::{info,warn,debug,error,trace};
use log4rs;

pub struct ServerConnecter{
    server_ip:String,
    control_port:u16,
    client_id:i32,
    connecting:bool,
    selfIp:String,
    selfDataPort:i32,
    to_server:Option<TcpStream>,
}

//note: by lyf
static mut sta_server_ip:String = String::new();
static mut sta_control_port:u16 = 0;
impl ServerConnecter{

    pub fn new(c_id:i32,selfIp:String,selfDataPort:i32)->ServerConnecter{//client.SynItem s
        ServerConnecter{
            /*server_ip: String::new(),
            control_port: 0,*/ //note:by lyf
            
            server_ip: unsafe{sta_server_ip.clone()},
            control_port:unsafe{ sta_control_port},
            
            client_id: c_id,
            connecting: true,
            selfIp:selfIp,
            selfDataPort:selfDataPort,
            to_server:None,
        }
    }

    pub fn init(/*&mut self,*/ s_ip:&String, c_port:&u16){
        /*self.server_ip = (*s_ip).clone();
        self.control_port = *c_port;*/ //note: by lyf
        unsafe{
        sta_server_ip = (*s_ip).clone();
        sta_control_port = *c_port;
        }
    }

    pub fn run(&mut self,status1:Arc<(Mutex<i32>,Condvar)>){

        let mut status = true;
        println!("SeverConnecter run!\n"); //note:by lyf
        info!("SeverConnecter run\n");
        while self.connecting{
            //println!("server_ip:{},control_port:{}",self.server_ip,self.control_port);//note:by lyf
            if let Ok(connect_socket) = TcpStream::connect((&self.server_ip[..], self.control_port)) {
                self.to_server = Some(connect_socket);
                println!("Connect to server successfully(control)!");
            } else {
                println!("Couldn't connect to server...");
                status = false;
            }
            
            if !status{
                break;
            }

            let mut input_buf = String::new();
            match &mut self.to_server{
                None => println!("Error! server not connected..."),
                Some (socket) => {
                    let socket_read = socket.try_clone().expect("clone failed...");
                    let mut in_from_server = BufReader::new(socket_read);
                    //dontpanic新加
                    socket.write_fmt(format_args!("3 {} {} {}\n", self.client_id.to_string(),self.selfIp,self.selfDataPort));
                    //TODO:err handle
                    socket.flush();//TODO:err
                    input_buf.clear();
                    in_from_server.read_line(&mut input_buf).unwrap();
                    //debug
                    println!("input_buf:{}",input_buf);

                    while self.connecting{
                        //我不知道原文件的client.Client.getRS()是什么东西所以没有写
                        socket.write_fmt(format_args!("1 {} {}\n", self.client_id.to_string(),crate::client::client::client::getRs()));//TODO:err handle
                        socket.flush();//TODO:err
                        input_buf.clear();
                        in_from_server.read_line(&mut input_buf).unwrap();
                        let input_buf = input_buf.trim();
                        println!("serverconnecter -- input_buf:{}\n",input_buf);
                        let mut input_vec:Vec<&str>= input_buf[..].split(' ').collect();

                        //sleep
                        let five_seconds = time::Duration::new(5, 0);
                        thread::sleep(five_seconds);
                    }
                
                }
            }


            match &mut self.to_server{
                None => println!("Error! server not connected..."),
                Some (socket) => {
                    socket.write(b"exit\n");
                    socket.flush();
                    socket.shutdown(Shutdown::Both)
                        .expect("socket shutdown call failed");
                }
            }
        }
        if self.connecting {
            //syn.setStatus(1);

            let &(ref lock, ref cvar) = &*status1;
            let mut status_cur = lock.lock().unwrap();
            *status_cur = 1;
            cvar.notify_all();
            println!("notify main thread");

            println!("ERR: connect to server has been interrupted!");
        }
    }

    pub fn stopConnect(&mut self){
        self.connecting = false;
    }

}