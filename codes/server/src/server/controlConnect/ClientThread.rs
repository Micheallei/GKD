use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::BufReader;
use std::io::BufRead;
use std::io::prelude::*;
use std::convert::TryInto;
use std::time::Duration;

use super::super::database::Query::Query;

pub struct ClientThread {
    client_socket:TcpStream,
    client_id:i32,
}

impl ClientThread{
    pub fn new(stream:TcpStream)->ClientThread{
        ClientThread{
            client_socket:stream,
            client_id:-1,
        }
    }

    fn readsentence(&mut self, sentence:&String) -> i32{
        //println!("control Connect - ClientThread :sentence:{}",sentence);
        let mut first_char = sentence.chars().next();
        match first_char {
            None => return 0,
            Some(c) =>{
                if c == '1'{
                    let s: Vec<&str> = sentence.split(' ').collect();
                    let id: i32 = s[1].trim().parse().unwrap();

                    if self.client_id != -1 && self.client_id != id{
                        self.client_socket.write(b"Error!\n");
                        self.client_socket.flush();
                        return 0;
                    }
                    //println!("controlconnect--clientthread--准备修改device");
                    let client_addr = self.client_socket.peer_addr().unwrap();
                    let port = client_addr.port();
                    let rs: i32 = s[2].trim().parse().unwrap();
                    let ip = client_addr.ip();

                    //println!("controlconnect--queryDevice所用的id：{}",id);
                    let query = Query::new();
                    let mut deviceitem = query.queryDevice(id);

                    self.client_id = id;
                    deviceitem.set_ip(ip.to_string());
                    deviceitem.set_port(port.try_into().unwrap());
                    deviceitem.set_is_online(true);
                    deviceitem.set_rs(rs);
                    if query.alterDevice(deviceitem) == -1 {
                        println!("alterDevice fail");
                    }

                    //println!("controlconnect--queryrequestnumber_byid所用id:{}",id);
                    self.client_socket.write_fmt(format_args!("received with {} unread request!\n", query.queryRequestNumbers_Byid(id)));
                    self.client_socket.flush();
                    //query.closeConnection();
			        return 1
                }
                else if c == '2'{
                    let s: Vec<&str> = sentence.split(' ').collect();
                    let id: i32 = s[1].trim().parse().unwrap();

                    if self.client_id != -1 && self.client_id != id{
                        self.client_socket.write(b"Error!\n");
                        self.client_socket.flush();
                        return 0
                    }
                    let query = Query::new();
                    let mut request = query.queryFirstRequest_Byid(id);
                    //query.closeConnection();
                    println!("{} {} {}\n", 
                        request.get_id(), request.get_fragment_id(), request.get_type());
                    self.client_socket.write_fmt(format_args!("{} {} {}\n", 
                        request.get_id(), request.get_fragment_id(), request.get_type()));
                    self.client_socket.flush();
                    return 1

                }
            },
        };
        0
    }

    /*pub fn run(&mut self){
        //println!("start!");
        self.client_socket.set_read_timeout(Some(Duration::new(60, 0))).expect("set_read_timeout call failed");
        self.client_socket.set_write_timeout(Some(Duration::new(60, 0))).expect("set_read_timeout call failed");
        let stream_clone = self.client_socket.try_clone().expect("clone failed...");
        let mut in_from_client = BufReader::new(stream_clone);
        //println!("control Connect - ClientThread :before loop!/n");
        loop{
            let mut sentence = String::new();
            sentence.clear();
            in_from_client.read_line(&mut sentence).unwrap();
            //println!("control Connect - ClientThread ：after read line!\n");
            if self.readsentence(&sentence) == 0 {
                break;
                //println!("control Connect - ClientThread ：sentence：{}，break\n",sentence);
            }
            println!("C-RECV: {}", sentence);
        }
        if self.client_id != -1 {
            /*let query = database::Query::new();
            let deviceitem = query.queryDevice(client_id);
            deviceitem.setIsOnline(false);
			query.alterDevice(deviceitem);
			query.closeConnection();*/
        }
        println!("C-client thread ended");
    }*/

    pub fn run(&mut self){
        //println!("start!");
        self.client_socket.set_read_timeout(Some(Duration::new(60, 0))).expect("set_read_timeout call failed");
        self.client_socket.set_write_timeout(Some(Duration::new(60, 0))).expect("set_read_timeout call failed");
        let stream_clone = self.client_socket.try_clone().expect("clone failed...");
        let mut in_from_client = BufReader::new(stream_clone);
        //println!("control Connect - ClientThread :before loop!/n");
        loop{
            let mut sentence = String::new();
            sentence.clear();
            let result = in_from_client.read_line(&mut sentence);
            if let Err(e) = result {
                println!("client break down");
                break;
            }
            //println!("control Connect - ClientThread ：after read line!\n");
            if self.readsentence(&sentence) == 0 {
                //println!("control Connect - ClientThread ：sentence：{}，break\n",sentence);
                break;
                //println!("control Connect - ClientThread ：sentence：{}，break\n",sentence);
            }
            println!("C-RECV: {}", sentence);
        }
        if self.client_id != -1 {
            let query = Query::new();
            let mut deviceitem = query.queryDevice(self.client_id);
            deviceitem.set_is_online(false);
            query.alterDevice(deviceitem);
            //query.closeConnection();
        }
        println!("C-client thread ended");
    }


}