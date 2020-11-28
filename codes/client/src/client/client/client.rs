use std::thread; 
use std::sync::{Arc, Mutex, Condvar};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

static mut sta_rs:i32 = -1;

pub fn main() {
    let mut clientId:i32 = 0;
    //let mut uploadFolders:Vec<PathBuf> = Vec::new();
    //let mut uploadAddrs:Vec<String> = Vec::new();
    let mut selfIp:String=String::new();
    let mut selfDataPort:i32 = 0;
    let mut rs:i32 = 0;
    println!("client start");


    //read setup.ini 
    let mut serverControlPort:i32 = 0;
        
    let setUpFile = String::from("D:\\setup.ini");
    let file = File::open(setUpFile).unwrap();
    println!("open setup.ini successfully!");

    let mut fin = BufReader::new(file);
    let mut line = String::new();

    fin.read_line(&mut line).unwrap(); 
    let mut serverIp = String::from(line.trim());
    println!("serverIp:{}",serverIp);
    

    line.clear();
    fin.read_line(&mut line).unwrap(); 
    let mut serverControlPort = line.trim().parse::<i32>().unwrap();
    println!("servercontrolPort:{}",serverControlPort);
    
    // note by lyf:因为后面新建的两个线程都用到此IP，为了避免move的问题，创建两个变量存一样的IP
    fin.read_line(&mut line).unwrap(); 
    let mut self_ServerConnect_Ip = String::from(line.trim());
    let self_RequestManager_Ip = self_ServerConnect_Ip.clone();
    println!("selfIp:{}",self_ServerConnect_Ip);

    line.clear();
    fin.read_line(&mut line).unwrap(); 
    let mut selfDataPort = line.trim().parse::<i32>().unwrap();
    println!("selfdataPort:{}\n",selfDataPort);

    line.clear();
    fin.read_line(&mut line).unwrap(); 
    clientId = line.trim().parse::<i32>().unwrap();

    //空行
    fin.read_line(&mut line).unwrap(); 
    
    line.clear();
    fin.read_line(&mut line).unwrap(); 
    let mut fragmentFolder = String::from(line.trim());

    line.clear();
    fin.read_line(&mut line).unwrap(); 
    unsafe{sta_rs = line.trim().parse::<i32>().unwrap();}
    unsafe{println!("rs:{}\n",sta_rs);}
    //setup 返回

    /*dontpanic组中代码无此部分
    line.clear();
    fin.read_line(&mut line).unwrap(); 
    let mut tmpFragmentFolder = String::from(line.trim());
    */
    /*dontpanic注释了关于uploadFolder的部分，其作为client的成员变量，可能会有其他代码调用然后赋值
    line.clear();
    fin.read_line(&mut line).unwrap(); 
    let i = line.trim().parse::<i32>().unwrap(); //需监控的上传文件夹数量


    let mut j = i;
    while j>0 {
        line.clear();
        fin.read_line(&mut line).unwrap(); 
        let uploadFolder = PathBuf::from(line.trim());
        uploadFolders.push(uploadFolder);

        line.clear();
        fin.read_line(&mut line).unwrap(); 
        let uploadAddr = String::from(line.trim());
        uploadAddrs.push(uploadAddr);
        j-=1;
    }
    */
    crate::client::connect::ServerConnecter::ServerConnecter::init(&serverIp,&(serverControlPort as u16));
    let mut file1 = PathBuf::from(&fragmentFolder);
    if !file1.exists() || !file1.is_dir(){
        println!("file1 wrong");
        return;
    }

    crate::client::connect::FragmentManager::FragmentManager::init(&file1);
    let mut file2 = PathBuf::from(&fragmentFolder);
    if !file2.exists() || !file2.is_dir(){
        println!("file2 wrong");
        return;
    }

    //crate::client::fileDetector::FolderScanner::FolderScanner::init(&file2);
    //crate::client::fileDetector::FolderScanner::FolderScanner::init(&tmpFragmentFolder);
    //crate::client::fileDetector::FileUploader::FileUploader::init(&file2,&serverIp,&(dataPort as u16)); //note:(by lyf) 类型转换
    //crate::client::fileDetector::FileUploader::FileUploader::init(&tmpFragmentFolder,&serverIp,&(dataPort as u16)); //note:(by lyf) 类型转换
    //note:by lyf  由于全局变量pathbuf类型难以实现，故传String

    //线程创建
    let status = Arc::new((Mutex::new(0), Condvar::new()));
    let connect_status = status.clone();
    let fileDetector_status = status.clone();//Arc<Mutex<i32>,Condvar>

    //let clientid = self.clientId.clone();

    let handle1 = thread::spawn(move || { 
        let mut ServerConnecter = crate::client::connect::ServerConnecter::ServerConnecter::new(clientId,self_ServerConnect_Ip.clone(),selfDataPort);
        ServerConnecter.run(connect_status);
     });//let mut num = counter.lock().unwrap(); *num += 1;
    
     /*dontpanic无此代码
    let handle2 = thread::spawn(move || {
    let folderScanner = crate::client::fileDetector::FolderScanner::FolderScanner::new(uploadFolders,uploadAddrs);
    folderScanner.run(fileDetector_status);
    });*/

    let handle2 = thread::spawn(move || { //note by lyf:requestManager声明为mut是因为websocket需要一个mut
        let mut requestManager = crate::client::connect::RequestManager::RequestManager::new(selfDataPort,self_RequestManager_Ip);
        requestManager.run(fileDetector_status);
        });

    let &(ref lock, ref cvar) = &*status;
    let mut status_cur = lock.lock().unwrap();
    while *status_cur==0 {//状态码未被改变时，则继续wait
        println!("before wait");
        status_cur = cvar.wait(status_cur).unwrap();
        println!("after wait");
    }

    /*NOTE:锁处理这块不太懂*/

    if *status_cur==1 {
        println!("Err: can not connect to server");
    }else if *status_cur==2{
        println!("Err: can detect files");
    }
}

//java中按类实现，但全写在main里rs是main中定义的变量，此处有bug
pub fn getRs() -> i32 {
    //返回剩余容量,待实现
	unsafe{return sta_rs;}
}
