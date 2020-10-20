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
use std::fs;


//note:by lyf
static mut sta_fragmentFolder:String = String::new();
static mut sta_serverIP:String = String::new();
static mut sta_serverPort:i32 = -1;
//dontpanic新增
static mut sta_selfPort:i32 = -1;


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
    user:WebSocket
}


impl FragmentManager {
    pub fn new(&self,rId : i32, fId : i32, t : i32)->FragmentManager{
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
            Type : t,//type为Rust关键字，改为大写开头
			user:None	//新添的websocket部分，在另一构造方法中初始化
        }
    }
		//原dontpanic组中此为另一构造方法，此处改为new_user()
	pub fn new_user(&mut self,iUser:WebSocket){
		self.user = iUser;
	}

    /*pub fn init0(&mut self, tmp : String, ip : String, port : i32){
        self.fragmentFolder = tmp;
        self.serverIP = ip;
        self.serverPort = port;
    }*/

    pub fn run(&mut self){
        // 暂不进行并发数据操作
        //websocket的recv（）返回byte[]，这里可能有bug
			let msg:String = String::new(user.recv());
			println!("msg:{}",msg);
			//TODO token
			//字符串比较相等，我忘了怎么比较了……，可能有bug
			if(msg=="U"){
				println!("Upload");
				let fileName:String = String::new(user.recv());
				println!("{}",fileName);
				self.recvDigest(fileName);
				self.recvFragment(fileName);
			} else if(msg == "D"){
					println!("Download");
					let fileName:String = String::new(user.recv());
					println!("{}",fileName);
					self.sendFragment(fileName);
					self.sendDigest(fileName);
			} else if (msg == "E") {
					println!("Echo");
					self.user.echo();
			} else {
					println!("Undefined operation");
			}
			self.user.close();
    }

	/*dontpanic删除了submit
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
		*/
		//dontpanic：init参数改变
    pub fn init(f : &PathBuf) {
        //note:by lyf
        /*self.fragmentFolder = f;
        self.serverIP = ip;
        self.serverPort = port;*/
        unsafe{
        sta_fragmentFolder = (*f).to_str().unwrap().to_string();
        //sta_serverIP = (*ip).clone();
        //sta_serverPort = *port;
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
    fn sendFragment(&mut self,fileName:String) -> bool {
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
        //s.push_str(&self.fragmentID.to_string());
        s.push_str(&fileName);
			println!("{}", s);
        let mut path = Path::new(&s);
        //可能会根据运行平台的不同添加/,分为posix和windows
        let mut f = File::open(&path).unwrap();

        //@SuppressWarnings("deprecation")
        /*match &mut self.toServer {
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
        }*/
			self.user.sendFile(f);
			status = true;   //TODO
        return status;

    }

    fn recvFragment(&mut self,fileName:String) -> bool {
        let mut s = String:: new();
        s.push_str(&self.fragmentFolder);
        s.push('/');
        //s.push_str(&self.fragmentID.to_string());
			s.push_str(&fileName);
        let mut path = Path::new(&s);
        //可能会根据运行平台的不同添加/,分为posix和windows
        let mut f = File::create(&path).unwrap();
        //remove_file(path);
			//dontpanic删除
			/*
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
        }*/
			self.user.recvFile(f);
			self.user.sendMessage("fragment success");
			println!("recvFragment {}",fileName);
        return true;//TODO
    }

		
	 fn sendDigest(&self,fileName:String) -> bool {
		let mut status:bool = false;
		let mut sentence:String;

		let mut s = String::new();
      s.push_str(&self.fragmentFolder);
      s.push('/');
		s.push_str(&fileName);
		s.push_str(".digest");
        let mut path = Path::new(&s);
		let mut f = File::open(&path).unwrap();
		self.user.sendMessage(fs::read_to_string(path));
		return true;
	 }

	fn recvDigest(&self,fileName:String) -> bool{
		let mut s = String::new();
        s.push_str(&self.fragmentFolder);
        s.push('/');
		s.push_str(&fileName);
		s.push_str(".digest");
		let mut path = Path::new(&s);
		//let mut f = File::open(&path).unwrap();
		//dontpanic中recv_bytes为byte[]类型
		let mut recv_bytes:Vec<u8> = self.user.recv();
		println!("recvDigest : {:?}",recv_bytes);//不确定能否输出string
        fs::write(path,recv_bytes); 
        self.user.sendMessage("digest success");
        return true;//TODO
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
        /*dontpanic删除
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

        }*/
        
        return true;//不知道为什么最后不加返回值就报错
    }


    fn errorHandler(Type : i32){
        return;
    }
    
}

