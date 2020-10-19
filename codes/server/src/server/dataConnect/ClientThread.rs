use std::string::String;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;
use chrono::Local;
use rand::Rng;
use std::path::PathBuf;
use std::fs::File;
use std::path::Path;
use std::time::Duration;

use super::super::database::Query::Query;
use super::super::database::Query::FileItem;
use super::super::database::Query::RequestItem;

/*在crate root 中声明 "extern crate chrono;"
cargo.toml中增加：
[dependencies]
chrono = "0.4"
rand = "0.6.0"
*/

//关于null的处理还没有确定，如何调用其他文件中的方法或函数还没有确定
//一部分对null特别处理的代码中，假定变量的类型是Option<T>
//需要参照其他对应文件

pub struct ClientThread{
    client_socket: TcpStream,
    //in_from_server:String,
    //out_to_client:String,
    sentence: String,
    download_folder_path: PathBuf,
    upload_folder_path: PathBuf,
}

impl ClientThread{
    pub fn new(stream:TcpStream)->ClientThread{
        ClientThread{
            client_socket: stream,
            sentence: String::new(),
            download_folder_path: PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/downloadFragment/"),
            upload_folder_path: PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/uploadFragment/"),
        }
    }

    pub fn run(mut self){
        let mut status:bool = false;
        //println!("start!");
        self.client_socket.set_read_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout call failed");
        self.client_socket.set_write_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout call failed");
        let in_from_client = self.client_socket.try_clone().expect("clone failed...");
        let mut in_from_client = BufReader::new(in_from_client);
        self.sentence.clear();
        in_from_client.read_line(&mut self.sentence).unwrap();
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        println!("D-RECV: {} {} {}", command[0], command[1], command[2]);
        println!("test:D-RECV:{}",self.sentence);

        status = match command[0] {
            "1" => self.recv_required_fragment(),
            "2" => self.send_fragment(),
            "3" => self.delete_fragment(),
            "4" => self.register_file(),
            "5" => self.recv_file_fragment(),
            "6" => self.check_folder(),
            _ => {
                self.client_socket.write(b"ERROR!\n");
                self.client_socket.flush();
                false
            },
        };

        if status{
            println!("D-client thread ended (finished)");
        }
        else{
            println!("D-client thread ended (aborted)");
        }
    }

    /*pub fn recv_required_fragment(mut self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].trim().parse().unwrap();

        let query = Query::new();
        let mut request = query.queryRequest_Byid(id);

        if request.get_fragment_id() != fid || request.get_type() != 1{
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            status = false;
        }
        else{
            println!("else\n");
            let mut s: String = self.download_folder_path.into_os_string().into_string().unwrap();
            s.push_str(&fid.to_string());
                //+ &fid.to_string();
            let recv_file = File::create(s).unwrap();
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
            status = super::FileTransporter::recv_file(recv_file, &self.client_socket);
            println!("status:{}\n", status);
            /*if status {
                self.client_socket.write(b"received!\n");
                self.client_socket.flush();
                query.deleteRequest(request.get_id());
            }*/
            if status {
                self.client_socket.write(b"received!\n");
                self.client_socket.flush();
                query.deleteRequest(request.get_id());
            } else {
                let mut f2 = File::create(&s).unwrap();
                let mut socket2 = self.client_socket.try_clone().expect("clone failed");
                if(super::FileTransporter::recv_file(f2, &socket2)) {
                    socket2.write(b"received!\n");
                    socket2.flush();
                    query.deleteRequest(request.get_id());
                }
            }
        }
        //query.closeConnection();
        status
    }*/

    pub fn recv_required_fragment(mut self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].trim().parse().unwrap();

        let query = Query::new();
        let mut request = query.queryRequest_Byid(id);

        if request.get_fragment_id() != fid || request.get_type() != 1{
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            status = false;
        }
        else{
            let mut s: String = self.download_folder_path.into_os_string().into_string().unwrap();
            s.push_str(&fid.to_string());
            //+ &fid.to_string();
            let recv_file = File::create(&s).unwrap();
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
            status = super::FileTransporter::recv_file(recv_file, &self.client_socket);
            if status {
                self.client_socket.write(b"received!\n");
                self.client_socket.flush();
                query.deleteRequest(request.get_id());
            } else {
                let mut f2 = File::create(&s).unwrap();
                let mut socket2 = self.client_socket.try_clone().expect("clone failed");
                if(super::FileTransporter::recv_file(f2, &socket2)) {
                    socket2.write(b"received!\n");
                    socket2.flush();
                    query.deleteRequest(request.get_id());
                }
            }
        }
        //query.closeConnection();
        status
    }

    pub fn send_fragment(mut self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].trim().parse().unwrap();
        
        let query = Query::new();
        let mut request = query.queryRequest_Byid(id);

        if request.get_fragment_id() != fid || request.get_type() != 2 {
            status = false;
        }
        else{
            let mut s = PathBuf::new();
            s.push(self.upload_folder_path);
            //let mut s: PathBuf = self.upload_folder_path.into_os_string().into_string().unwrap();
            s.push(&fid.to_string());
            //let mut s: String = self.upload_folder_path.into_os_string().into_string().unwrap();
            //s.push_str(&fid.to_string());
            let send_file = File::open(&s);
            //test
            println!("send_file_path:{}",s.clone().as_path().display());
            
            match send_file{
                Err(e) => {
                    status = false;
                    query.deleteRequest(request.get_id());
                },
                Ok(file) =>{
                    
                    status = super::FileTransporter::send_file(file, &self.client_socket);
                    if status{
                        let mut in_from_cilent = BufReader::new(self.client_socket);
                        let mut sentence = String::new();
                        in_from_cilent.read_line(&mut sentence).unwrap();
                        //println!("sentence after send file:{}",sentence);
                        let re = vec!['r', 'e', 'c', 'e', 'i', 'v', 'e', 'd', '!','\n'];
                        let mut n: usize = 0;
                        for sen in sentence.chars() {
                            if sen != re[n] {break;}
                            else {n = n + 1;}
                        }
                        //println!("n={},re,len={}",n,re.len());
                        if n == re.len() {
                            //sendFile.delete();
                            if query.deleteRequest(request.get_id()) == -1{
                                println!("deleteRequest fail!");
                            };
                            //query.alterFragment(fid, Integer.toString(request.getDeviceId()));
                            query.alterFragment(fid, request.get_device_id().to_string());
                        }
                    } else {
                        println!("!!send error");
                        let mut f2 = File::open(&s).unwrap();
                        let socket2 = self.client_socket.try_clone().expect("clone failed");
                        if(super::FileTransporter::send_file(f2, &socket2)) {
                            let mut in_from_cilent = BufReader::new(socket2);
                            let mut sentence = String::new();
                            in_from_cilent.read_line(&mut sentence).unwrap();
                            //println!("sentence after send file:{}",sentence);
                            let re = vec!['r', 'e', 'c', 'e', 'i', 'v', 'e', 'd', '!','\n'];
                            let mut n: usize = 0;
                            for sen in sentence.chars() {
                                if sen != re[n] {break;}
                                else {n = n + 1;}
                            }
                            //println!("n={},re,len={}",n,re.len());
                            if n == re.len() {
                                //sendFile.delete();
                                if query.deleteRequest(request.get_id()) == -1{
                                    println!("deleteRequest fail!");
                                };
                                //query.alterFragment(fid, Integer.toString(request.getDeviceId()));
                                query.alterFragment(fid, request.get_device_id().to_string());
                            }
                        } else {
                            println!("send file failed again");
                        }
                    }
                }
            };
            
        }
        //query.closeConnection();
        status
    }

    pub fn delete_fragment(&mut self)->bool{
        let mut status:bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let id:i32 = command[1].parse().unwrap();
        let fid:i32 = command[2].parse().unwrap();

        let query = Query::new();
        let mut request = query.queryRequest_Byid(id);

        if request.get_fragment_id() != fid || request.get_type() != 3 {
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            //query.closeConnection();
            status = false;
        }
        else{
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
            query.deleteRequest(request.get_id());
            //query.closeConnection();
        }
        status
    }

    pub fn register_file(&mut self)->bool{
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let noa:i32 = command[5].trim().parse().unwrap();
        let isf:bool = command[6].trim().parse().unwrap();

        let query = Query::new();
        let dt = Local::today();
        let mut date:String = dt.to_string();
        date.truncate(10);
        date.remove(7);
        date.remove(4);
        let fileitem = FileItem::init_2(command[2][..].to_string(), command[3][..].to_string(),
        command[4][..].to_string(), date, -1 * noa, isf);

        let fid = query.addFile(fileitem);
        
        self.client_socket.write_fmt(format_args!("FileId: {}\n", fid));
        self.client_socket.flush();

        //query.closeConnection();
        true
    }

    pub fn recv_file_fragment(mut self)->bool{
        let mut status: bool = true;
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        let file_id:i32 = command[1].parse().unwrap();
        let fragment_num:i32 = command[2].parse().unwrap();
        let fragment_count:i32 = command[3].trim().parse().unwrap();

        let query = Query::new();
        let mut file = query.queryFile_Byid(file_id);

        if file.get_noa() != -1 * fragment_count || fragment_num >= fragment_count || fragment_num < 0 {
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
            status = false;
        }
        else{
            let temp = file_id * 100 + fragment_num;
            //let mut s: String = self.upload_folder_path.into_os_string().into_string().unwrap();
            let mut s: String = self.upload_folder_path.clone().into_os_string().into_string().unwrap();
            let mut s1: String = self.upload_folder_path.clone().into_os_string().into_string().unwrap();
            //let mut s1: String = s.clone();
            s.push_str(&temp.to_string());
            println!("{}",s);
            let recv_file = File::create(&s).unwrap();
            self.client_socket.write(b"received!\n").unwrap();
            self.client_socket.flush();
            //println!("dataConnect--recv_file_fragment:after received!");
            //test:by lyf
            //self.client_socket.write(b"received!\n");
            //self.client_socket.flush();

            status = super::FileTransporter::recv_file(recv_file, &self.client_socket);
            //println!("status: {}", &status);
            if status{
                query.addFragment(temp, "-1".to_string());
                if fragment_num == fragment_count - 1 {
                    let count = query.queryFragmentNumbers(file_id);
                    if count == fragment_count && self.confirm(&file_id, &fragment_count) == 1{
                        self.client_socket.write(b"received!\n");
                        self.client_socket.flush();
                        file.set_noa(fragment_count);
                        query.alterFile(file);
                    }
                    else{
                        self.client_socket.write(b"UPLOADFAIL!\n");
                        self.client_socket.flush();
                        query.deleteFile(file_id);
                        for i in 0..fragment_count{
                            if query.deleteFragment(file_id * 100 + i) == 1 {
                                //let temp_2:i32 = file_id * 100 + i;
                                //s1.push_str(&temp_2.to_string());
                                let temp_2:String = (file_id * 100 + i).to_string();
                                let f = File::create(s1.clone()+&temp_2).unwrap();
                            }
                        }
                    }
                }
                else{
                    self.client_socket.write(b"received!\n");
                    self.client_socket.flush();
                }
            } else {
                println!("!!recv file failed");
                let recv_file2 = File::create(&s).unwrap();
                let mut socket2 = self.client_socket.try_clone().expect("clone failed");
                if(super::FileTransporter::recv_file(recv_file2, &socket2)) {
                    query.addFragment(temp, "-1".to_string());
                    if fragment_num == fragment_count - 1 {
                        let count = query.queryFragmentNumbers(file_id);
                        if count == fragment_count && self.confirm(&file_id, &fragment_count) == 1{
                            socket2.write(b"received!\n");
                            socket2.flush();
                            file.set_noa(fragment_count);
                            query.alterFile(file);
                        }
                        else{
                            socket2.write(b"UPLOADFAIL!\n");
                            socket2.flush();
                            query.deleteFile(file_id);
                            for i in 0..fragment_count{
                                if query.deleteFragment(file_id * 100 + i) == 1 {
                                    //let temp_2:i32 = file_id * 100 + i;
                                    //s1.push_str(&temp_2.to_string());
                                    let temp_2:String = (file_id * 100 + i).to_string();
                                    let f = File::create(s1.clone()+&temp_2).unwrap();
                                }
                            }
                        }
                    }
                    else{
                        socket2.write(b"received!\n");
                        socket2.flush();
                    }
                } else {
                    println!("recv file failed again");
                }
            }
        }
        //query.closeConnection();
        status
    }

    pub fn check_folder(mut self)->bool{
        //println!("enter check folder");
        let command:Vec<&str> = self.sentence[..].split(' ').collect();
        //println!("command:{:?}",command);
        let num:i32 = command[2].trim().parse().unwrap();
        //println!("num:{}",num);

        let query = Query::new();
        let mut flag: bool = false;
        let mut i = 0;
        for i in 0..num {
            //println!("{} {}",command[(3+2*i) as usize], command[(4+2*i) as usize]);
            let mut file = query.queryFile_Bypathname(Some(command[(3+2*i) as usize].to_string()), 
                Some(command[(4+2*i) as usize].to_string()));
            //println!("fileid: {}", file.get_id());
            if  0 == file.get_id() {
                //println!("no file!");
                let dt = Local::today();
                let mut date:String = dt.to_string();
                date.truncate(10);
                date.remove(7);
                date.remove(4);
                let file = FileItem::init_2(command[(4+2*i) as usize].to_string(), command[(3+2*i) as usize].to_string(),
                    "rw".to_string(), date, 0, true);
                if query.addFile(file) < 0{
                    flag = true;
                }
            } else {
                if !file.is_folder() {
                    flag = true;
                }
            }
            if flag {
                break;
            }
        }

        //println!("i:{}\n",i);
        i = i + 1;
        if i == num {
            println!("received");
            self.client_socket.write(b"received!\n");
            self.client_socket.flush();
        }
        else {
            println!("ERROR");
            self.client_socket.write(b"ERROR!\n");
            self.client_socket.flush();
        }

        //query.closeConnection();
        true
    }

    pub fn confirm(&mut self, id:&i32, num:&i32)->i32{
        let query = Query::new();
        //let mut return_val:i32 = 0;

        let mut di = query.queryOnlineDevice();
        //假定di类型为Vec<DeviceItem>
        if di.is_empty() {
            return -1;
        }

        let s = di.len();
        let size: i32 = s as i32;
        if num <= &size {
            let t: i32 = rand::thread_rng().gen_range(0, size);
            for i in 0..*num{
                let n: i32 = i as i32;
                let temp = RequestItem::init_2(2, id * 100 + n, di[((n + t) % size) as usize].get_id());
                query.addRequest(temp);
            }
        }
        else{
            let mut n:Vec<i32> = Vec::new();
            let temp = num / size;
            for i in 0..size {
                n.push(temp);
            }
            let m = num % size;

            let mut t = rand::thread_rng().gen_range(0, size);
            for i in 0..m {
                n[(t % size) as usize] = n[(t % size) as usize] + 1;
                t = t + 1;
            }

            let mut k:i32 = 0;
            for i in 0..size {
                for j in 0..n[i as usize] as usize{
                    let temp = RequestItem::init_2(2, id * 100 + (k as i32), di[i as usize].get_id());
                    query.addRequest(temp);
                    k = k + 1;
                }
            }
        }
        return 1
    }
}