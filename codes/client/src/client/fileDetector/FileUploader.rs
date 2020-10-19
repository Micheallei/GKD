
use std::path::PathBuf;
use std::net::TcpStream;
use std::string::String;
use std::io::prelude::*;
use std::fs::read_to_string;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

use super::FileAttrs::FileAttrs;

pub struct FileUploader {
    serverIP: String,
    server_port: u16,
    tmpFragmentFolder: PathBuf,
    to_server: Option<TcpStream>,
    connecting: bool,
}

static mut sta_serverIP:String = String::new();
static mut sta_server_port:u16 = 0;
// lazy_static!{
//     static ref sta_tmpFragmentFolder:PathBuf = {
//         let mut m=PathBuf::new();
//         m
//     }; //PathBuf::from("foo.txt");
// }
//static mut sta_tmpFragmentFolder:PathBuf = PathBuf::new();
static mut sta_tmpFragmentFolder:String = String::new();

impl FileUploader {
    pub fn init(f:/*&PathBuf*/&String, ip: &String, port:&u16)/* -> Self*/{
        //note:(by lyf)参数改成了引用
        //note:by lyf init改为对
        /*FileUploader {
            serverIP: (*ip).to_string()/*.clone()*/,
            server_port: *port,
            tmpFragmentFolder: (*f.clone()).to_path_buf(),
            connecting: false,
            to_server: None,
        }*/
        unsafe{
        sta_serverIP = (*ip).clone().to_string();
        sta_server_port = *port;
        sta_tmpFragmentFolder = (*f).clone().to_string();
        }

    }

    pub fn new() -> FileUploader{
        FileUploader {
            serverIP: unsafe{sta_serverIP.clone()},
            server_port:unsafe{sta_server_port},
            tmpFragmentFolder: unsafe{PathBuf::from(sta_tmpFragmentFolder.clone())},
            connecting: false,
            to_server: None,
        }
    }

    pub fn checkFolders(mut self, addr: &Vec<String>) -> bool{
        if !self.createConnection() {
            return false;
        }
        //println!("1\n");
        let addr:Vec<String> = (*addr).clone().to_vec();
        match &mut self.to_server {
            None => false,
            Some (socket) => {
                //println!("2\n");
                if !self.connecting
                    {return false;}
                socket.write_fmt(format_args!("6 0 {} ", addr.len()));
                //println!("6 0 {}\n", addr.len());
                socket.flush();

                let mut i = 0;
                while i < addr.len() {
                    let c = addr[i].chars();
                    let mut j = -1;
                    let mut n = 0;
                    for cur in c {
                        if cur == '/' {j = n;}
                        n = n+1;
                    }
                    if j==-1
                        {socket.write_fmt(format_args!("/ {} ", &addr[i]));}
                        //println!("no \\");
                        //println!("/ {}\n", addr[i]);
                    else {
                        let mut number = 0;
                        let ch = addr[i].chars();
                        for cur in ch {
                            socket.write_fmt(format_args!("{}\n", cur));
                            if number == j {socket.write_fmt(format_args!("/ "));}
                            number = number + 1;
                        }
                        socket.write_fmt(format_args!("\n"));
                    }
                    socket.flush();
                    //println!("send suc");
                    socket.write_fmt(format_args!("\n"));
                    socket.flush();
                    i = i + 1;
                }
                let re = ['r','e','c','e','i','v','e','d','!','\n'];
                let mut i = 0;
                let mut input_buf = String::new();
                socket.read_to_string(&mut input_buf);
                //println!("input: {}\n", input_buf);
                for c in input_buf.chars() {
                    if c == re[i] {i = i+1;}
                    else {return false;}
                }
                return true;
            }
        }
        
    }

    pub fn registerFile(&mut self, fa: FileAttrs) -> i32 {
        println!("enter registerfile\n");
        if !self.createConnection(){
            return -2;
        }
        match &mut self.to_server {
            None => 0,
            Some (socket) => {
                if !self.connecting {return -2;}
                socket.write_fmt(format_args!("4 0 "));
                socket.write_fmt(format_args!("{} {} {} {} false\n", fa.name, fa.path, fa.attr, fa.noa));
                socket.flush();

                let mut sentence = String::new();
                socket.read_to_string(&mut sentence);
                let mut input: Vec<char> = Vec::new();
                for cha in sentence.chars() {
                    if cha == ' ' {break;}
                    else {input.push(cha);}
                }
                let mut num = 0;
                for cha in "FileId:".chars() {
                    if cha != input[num] {return -2;}
                    num = num+1;
                }
                let mut inputline = String::new();
                num = 0;
                for cha in sentence.chars() {
                    if cha == ' ' {num = num + 1;}
                    if num == 2 {break;}
                    else if num == 1 {inputline.push(cha);}
                }
                let integer = inputline.trim().parse::<i32>().unwrap();
                return integer;
            }
        }
    }

    pub fn pushFragment(&mut self, fileId: i32, fragmentNum: i32, fragmentCount: i32) -> bool {
        println!("enter pushFragment\n");
        if !self.createConnection() {
            return false;
        }
        match &mut self.to_server {
            None => false,
            Some (socket) => {
                let mut status = false;
                let sentence = String::new();
                
                let mut f_path = PathBuf::new();
                f_path.push(&self.tmpFragmentFolder);
                f_path.push((fileId * 100 + fragmentNum).to_string());
                if !f_path.exists() {
                    println!("f create error --FileUploader pushFragment\n");
                    return false;
                }

                socket.write_fmt(format_args!("5 {} {} {}\n", fileId, fragmentNum, fragmentCount));
                socket.flush();

                let re = ['r','e','c','e','i','v','e','d','!','\n'];
                let mut i = 0;
                let mut inFromServer = String::new();
                //println!("Fileuploader--pushFragment:before read_to_string");
                //socket.read_to_string(&mut inFromServer);
                let in_from_server = socket.try_clone().expect("clone failed...");
                let mut in_from_server = BufReader::new(in_from_server);
                in_from_server.read_line(&mut inFromServer);


                //println!("Fileuploader--pushFragment:after read_to_string");
                //println!("inFromServer:{}",inFromServer);
                for c in inFromServer.chars() {
                    if c == re[i] {i = i+1;}
                    else {return false;}
                }

                //let mut f:File = File::create(&f_path.as_path()).unwrap();
                let mut f:File = File::open(&f_path.as_path()).unwrap();
                println!("传递给FileTransporter的file的path:{}",f_path.display());
                let socket = self.to_server.as_ref().unwrap();
                status = crate::client::connect::FileTransporter::send_file(f, &socket);
                println!("fileUploader--pushFragment--status(result of FileTransporter):{}",status);

                if status {
                    let re = ['r','e','c','e','i','v','e','d','!','\n'];
                    let mut i = 0;
                    for c in inFromServer.chars() {
                        if c == re[i] {i = i+1;}
                        else {return false;}
                    }
                }
                println!("push Fragment end");
                return status;
            }
        }
    }

    pub fn createConnection(&mut self) -> bool{
        if self.serverIP.is_empty(){
            return false;
        }
        if let Ok(connect_socket) = TcpStream::connect((&self.serverIP[..], self.server_port)) {
            connect_socket.set_read_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout call failed");
            connect_socket.set_write_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout call failed");
            self.to_server = Some(connect_socket);
            println!("Connect to server successfully(control)!");
            self.connecting = true;
        } else {
            println!("Couldn't connect to server...");
            self.connecting = false;
            return false;
        }

        return true;
    }
}