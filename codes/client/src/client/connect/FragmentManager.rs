//mod FileTransporter;

//use std::net::{TcpStream, Ipv4Addr, Shutdown};
use std::net::TcpStream;
use std::string::String;
use std::io::BufReader;
use std::io::Write;
use std::io::prelude::*;
//use std::{thread, time};
use std::fs::{File, remove_file};
//use std::ptr::null;
use std::path::Path;
use std::path::PathBuf;
//use std::option::NoneError;
use std::option::Option;
use std::time::Duration;


//note:by lyf
static mut sta_fragmentFolder:String = String::new();
static mut sta_serverIP:String = String::new();
static mut sta_serverPort:i32 = -1;

pub struct FragmentManager{
    fragmentFolder : String,
    serverIP : String,
    serverPort : i32,
    controlPort : u16,
    toServer : Option<TcpStream>,
    //inFromServer : BufReader<TcpStream>,
    requestID : i32,
    fragmentID : i32,
    Type : i32,
}


impl FragmentManager {
    pub fn new(rId : i32, fId : i32, t : i32)->FragmentManager{
        FragmentManager{
            /*fragmentFolder : String :: new(),
            serverIP : String :: new(),
            serverPort : -1,*/ //note:by lyf
            fragmentFolder :unsafe{ sta_fragmentFolder.clone()},
            serverIP : unsafe{sta_serverIP.clone()},
            serverPort :unsafe{ sta_serverPort},

            controlPort : 0,
            toServer : None,
            //inFromServer: BufReader :: new(l),
            requestID : rId,
            fragmentID : fId,
            Type : t//type为Rust关键字，改为大写开头
        }
    }

    /*pub fn init0(&mut self, tmp : String, ip : String, port : i32){
        self.fragmentFolder = tmp;
        self.serverIP = ip;
        self.serverPort = port;
    }*/

    pub fn run(){
        // 暂不进行并发数据操作
        // submit();
    }

    pub fn submit(&mut self) -> bool {
        let mut status = true;
        if self.serverIP.len() == 0 {
            return false;
        }
        if let Ok(connect_socket) = TcpStream::connect((&self.serverIP[..], self.serverPort as u16)) {
            connect_socket.set_read_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout call failed");
            connect_socket.set_write_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout call failed");
            self.toServer = Some(connect_socket);//忽略了setKeepAlieve和setsoTimeout，未找到rust中对应的长连接和超时连接的处理函数
            match &mut self.toServer {
                None => println!("Error"),
                Some(socket) => {
                    let socket_read = socket.try_clone().expect("clone failed");
                    let mut inFromServer = BufReader :: new(socket_read);
                }
            }
            println!("Connect to server successfully(data)!");
            if self.Type == 1 {
                status = self.sendFragment();
            } else if self.Type == 2 {
                status = self.recvFragment();
            } else if self.Type == 3 {
                status = self.deleteFragment();
            }
        } else {
            println!("Cannot connect to server");
            status = false;
        }
        return status;
    }

    pub fn init(/*&mut self,*/ f : &PathBuf/*String*/, ip : &String, port : &i32) {
        //note:by lyf
        /*self.fragmentFolder = f;
        self.serverIP = ip;
        self.serverPort = port;*/
        unsafe{
        sta_fragmentFolder = (*f).to_str().unwrap().to_string();
        sta_serverIP = (*ip).clone();
        sta_serverPort = *port;
        }
        /*match &mut self.toServer {
            None => println!("Error"),
            Some(socket) => {
                let socket_read = socket.try_clone().expect("clone failed");
                let mut inFromServer = BufReader :: new(socket_read);
            }
        }*/
    }
    //以下函数未实现throw exceptions
    fn sendFragment(&mut self) -> bool {
        let mut status = true;
        let mut sentense = String :: new();
        /*let mut pathBuf = PathBuf::new();
        pathBuf.push(&self.fragmentFolder);
        println!("{}", self.fragmentFolder);
        pathBuf.push("\\");
        pathBuf.push(&self.fragmentID.to_string());
        //可能会根据运行平台的不同添加/,分为posix和windows
        println!("{}", pathBuf.display());
        let mut f = File::open(pathBuf).unwrap();*/
        /*if !f.is_ok() {//如何判断一个文件是否存在？
            panic!("Error happens on File");
        }*/
        let mut s = String:: new();
        s.push_str(&self.fragmentFolder);
        s.push('\\');
        s.push_str(&self.fragmentID.to_string());
        println!("{}", s);
        let mut path = Path::new(&s);
        //可能会根据运行平台的不同添加/,分为posix和windows
        let mut f = File::open(&path).unwrap();

        //@SuppressWarnings("deprecation")
        match &mut self.toServer {
            None => println!("Error"),
            Some(socket) => {
                socket.write_fmt(format_args!("{} {} {}\n", self.Type, self.requestID, self.fragmentID));
                socket.flush();
                let socket1 = socket.try_clone().expect("clone failed");//克隆端口
                let socket2 = socket.try_clone().expect("clone failed");//克隆端口
                let mut inFromServer = BufReader::new(socket1);
                inFromServer.read_line(&mut sentense).unwrap();
                let recv = String :: from("received!\n");
                if !sentense.eq(&recv) {
                    return false;
                }
                
                let mut status : bool = super::FileTransporter::send_file(f, &socket2);
                //let mut status = FileTransporter.sendFile 需要另一个函数FileTransporter
                if status {
                    inFromServer.read_line(&mut sentense).unwrap();
                    if !sentense.eq(&recv) {
                        status = false;
                    }
                }
            }
        }
        return status;

    }

    fn recvFragment(&mut self) -> bool {
        let mut s = String:: new();
        s.push_str(&self.fragmentFolder);
        s.push('/');
        s.push_str(&self.fragmentID.to_string());
        let mut path = Path::new(&s);
        //可能会根据运行平台的不同添加/,分为posix和windows
        let mut f = File::create(&path).unwrap();
        //remove_file(path);
        println!("receive fragment");
        match &mut self.toServer {
            None => println!("Error"),
            Some(socket) => {
                let socket1 = socket.try_clone().expect("clone failed");//克隆端口
                println!("{} {} {}\n", self.Type, self.requestID, self.fragmentID);
                socket.write_fmt(format_args!("{} {} {}\n", self.Type, self.requestID, self.fragmentID));
                socket.flush();
                if (super::FileTransporter::recv_file(f, &socket1)){
                    socket.write_fmt(format_args!("received!\n"));
                    socket.flush();
                    println!("recvfragment success");
                    return true;
                }else {
                    //return false;
                    println!("!!recv failed");
                    let mut f2 = File::create(path).unwrap();
                    let socket2 = socket.try_clone().expect("clone failed");
                    if (super::FileTransporter::recv_file(f2, &socket2)){
                        socket.write_fmt(format_args!("received!\n"));
                        socket.flush();
                        println!("recvfragment success");
                        return true;
                    } else {
                        return false;
                    }
                }

            }
        }
        return true;//不知道为什么最后不加返回值就报错
    }

    fn deleteFragment(&mut self) -> bool {
        let mut s = String:: new();
        s.push_str(&self.fragmentFolder);
        s.push('/');
        s.push_str(&self.fragmentID.to_string());
        let mut path = Path::new(&s);
        //可能会根据运行平台的不同添加/,分为posix和windows
        let mut f = File::create(path).unwrap();
        remove_file(path);
        match &mut self.toServer {
            None => println!("Error"),
            Some(socket) => {
                socket.write_fmt(format_args!("{} {} {}\n", self.Type, self.requestID, self.fragmentID));
                socket.flush();
                //SuppressWarngings
                let socket1 = socket.try_clone().expect("clone failed");//克隆端口
                let mut inFromServer = BufReader::new(socket1);

                let mut sentense = String ::new();
                inFromServer.read_line(&mut sentense).unwrap();
                let recv = String :: from("received!");
                if sentense.eq(&recv) {
                    return true;
                }else {
                    return true;
                }
            }

        }
        return true;//不知道为什么最后不加返回值就报错
    }


    fn errorHandler(Type : i32){
        return;
    }
    
}

