//mod FileTransporter;

use std::net::TcpStream;
use std::string::String;
use std::io::BufReader;
use std::io::Write;
use std::io::prelude::*;
use std::fs::{File, remove_file};
use std::path::Path;
use std::path::PathBuf;
use std::option::Option;
use std::time::Duration;
use websocket::OwnedMessage;
use std::fs;
use crate::client::WebSocket::WebSocket::WebSocket;

use log::{info,warn,debug,error,trace};
use log4rs;

//note:by lyf
static mut sta_fragmentFolder:String = String::new();
static mut sta_serverIP:String = String::new();
static mut sta_serverPort:i32 = -1;
//dontpanic新增
static mut sta_selfPort:i32 = -1;


pub struct FragmentManager{
    fragmentFolder : String,
    user:WebSocket
}


impl FragmentManager {

		//原dontpanic组中此为另一构造方法，此处改为new_user()
	pub fn new_user(iUser:WebSocket) -> FragmentManager{
        FragmentManager{
            fragmentFolder :unsafe{ sta_fragmentFolder.clone()},
            user:iUser
        }
		
	}

    /*pub fn init0(&mut self, tmp : String, ip : String, port : i32){
        self.fragmentFolder = tmp;
        self.serverIP = ip;
        self.serverPort = port;
    }*/

    pub fn run(&mut self){
        // 暂不进行并发数据操作
        //websocket的recv（）返回byte[]，这里可能有bug
        info!("Start a new RequestManager ");
			let msg:String = match self.user.recv(){
                OwnedMessage::Text(data) => data,
                _ => String::from("dismatch")
            };
			println!("msg:{}",msg);
			//TODO token
			
			if(msg=="U"){
				println!("Upload");
				let fileName:String = match self.user.recv(){
                    OwnedMessage::Text(data) => data, 
                    _ => String::new()
                };
				println!("{}",fileName);
				self.recvDigest(fileName.clone());
				self.recvFragment(fileName.clone());
			} else if(msg == "D"){
					println!("Download");
					let fileName:String = match self.user.recv(){
                        OwnedMessage::Text(data) => data,
                        _ => String::new()
                    };
					println!("{}",fileName);
					self.sendFragment(fileName.clone());
					self.sendDigest(fileName.clone());
			} else if (msg == "E") {
					println!("Echo");
					self.user.echo();
			} else {
					println!("Undefined operation");
			}
			self.user.close();
    }


		//dontpanic：init参数改变
    pub fn init(f : &PathBuf) {
        unsafe{
        sta_fragmentFolder = (*f).to_str().unwrap().to_string();
        }

    }
    //以下函数未实现throw exceptions
    fn sendFragment(&mut self,fileName:String) -> bool {
        let mut status = true;
        let mut sentense = String :: new();
        let mut s = String:: new();
        s.push_str(&self.fragmentFolder);
        s.push('/');
        //s.push_str(&self.fragmentID.to_string());
        s.push_str(&fileName);
			println!("{}", s);
        let mut path = PathBuf::from(&s);
        //可能会根据运行平台的不同添加/,分为posix和windows
        // let mut f = File::open(&path).unwrap();

			self.user.sendFile(&path);
			status = true;   //TODO
        return status;

    }

    fn recvFragment(&mut self,fileName:String) -> bool {
        let mut s = String:: new();
        s.push_str(&self.fragmentFolder);
        s.push('/');
        //s.push_str(&self.fragmentID.to_string());
			s.push_str(&fileName);
        let mut path = PathBuf::from(&s);
        //可能会根据运行平台的不同添加/,分为posix和windows
        let mut f = File::create(&path).unwrap();

			self.user.recvFile(&path);
			self.user.sendMessage("fragment success".to_string());
			println!("recvFragment {}",fileName);
        return true;//TODO
    }

		
	 fn sendDigest(&mut self,fileName:String) -> bool {  //由于websocket需要mut 此处为&mut self
		let mut status:bool = false;
		let mut sentence:String;

		let mut s = String::new();
      s.push_str(&self.fragmentFolder);
      s.push('/');
		s.push_str(&fileName);
		s.push_str(".digest");
        let mut path = Path::new(&s);
		let mut f = File::open(&path).unwrap();
		self.user.sendMessage(fs::read_to_string(path).unwrap());
		return true;
	 }

	fn recvDigest(&mut self,fileName:String) -> bool{
		let mut s = String::new();
        s.push_str(&self.fragmentFolder);
        s.push('/');
		s.push_str(&fileName);
		s.push_str(".digest");
		let mut path = Path::new(&s);
		//let mut f = File::open(&path).unwrap();
		//dontpanic中recv_bytes为byte[]类型
		let mut recv_bytes:Vec<u8> = match self.user.recv(){
            OwnedMessage::Binary(data) => data,
            _ => Vec::new()
        };;
		println!("recvDigest : {:?}",recv_bytes);//不确定能否输出string
        fs::write(path,recv_bytes); 
        self.user.sendMessage("digest success".to_string());
        return true;//TODO
    }



    fn errorHandler(Type : i32){
        return;
    }
    
}

