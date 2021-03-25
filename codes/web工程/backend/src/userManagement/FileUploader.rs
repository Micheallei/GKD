use std::net::TcpStream;
use std::path::PathBuf;
use std::path::Path;
use std::string::String;
use std::io::prelude::*;
use std::fs::{self, File, read_to_string, DirEntry, remove_file};
use serde_json::{Result, Value, json};
use std::ffi::{OsStr, OsString};
use std::*;
use std::convert::TryInto;
//import com.opensymphony.xwork2.ActionSupport;
//import database.AnotherRequestItem;
use super::super::database::DeviceItem::DeviceItem;
use super::super::database::FileItem::FileItem;
use super::super::database::Query::Query;
use super::super::com::Decoder;
//use crate::userManagement::FileDownloader::FileDownloader;
use std::ptr::null;
use std::iter::once_with;

pub struct FileUploader{
    pub serialVersionUID: i32,
    pub path: String,
    pub fileName: String,
    pub result: String,
    pub devices: Value,
    pub fileType: String,
    pub fileSize: i32,
    pub fileblocks:i32,
    pub noa: i32,
    pub nod: i32,
    pub whose: String,
    pub fileId: i32,
    //fragmentFolderPath:PathBuf,
    //fileFolderPath:PathBuf,

}

//static mut fragmentFolderPath: String = String::from("/usr/local/tomcat/webapps/DFS/CloudDriveServer/downloadFragment");
//static mut fileFolderPath: String = String::from("/usr/local/tomcat/webapps/DFS/CloudDriveServer/tmpFile");
//以上两行可能要根据Tomcat具体安装路径来定

impl FileUploader{
    pub fn new() -> FileUploader{
        FileUploader{
            serialVersionUID: 1,
            path: String::new(),
            fileName: String::new(),
            result: String::new(),
            devices: serde_json::from_str("").unwrap(),//?用空字符串来初始化
            fileType: String::new(),
            fileSize: 0,
            fileblocks:0,
            noa: 0,
            nod: 0,
            whose: String::new(),
            fileId: 0,
            //fragmentFolderPath:PathBuf::from("/usr/local/tomcat/webapps/DFS/CloudDriveServer/downloadFragment"),
            //fileFolderPath:PathBuf::from("/usr/local/tomcat/webapps/DFS/CloudDriveServer/tmpFile"),
        }
    }
    pub fn getPath(&self) -> String{
        return self.path.clone();
    }

    pub fn setPath(&mut self, npath: String){
        self.path = npath;
    }

    pub fn getResult(&self) -> String{
        return self.result.clone();
    }

    pub fn setResult(&mut self, nresult: String){
        self.result = nresult;
    }

    pub fn getFileName(&self) -> String{
        return self.fileName.clone();
    }

    pub fn setFileName(&mut self, nfileName: String){
        self.fileName = nfileName;
    }

    pub fn getDevices(&self) -> Value{
        return self.devices.clone();//?
    }

    pub fn setDevices(&mut self, ndevices: Value){
        //let s: String = String::from(ndevices.to_string());
        //self.devices = serde_json:: from_str(&s);
        self.devices = ndevices;
    }

    pub fn getFileType(&self) -> String{
        return self.fileType.clone();
    }

    pub fn setFileType(&mut self, nfileType: String){
        self.fileType = nfileType;
    }

    pub fn getFileSize(&self) -> i32{
        return self.fileSize.clone();
    }

    pub fn setFileSize(&mut self, nfileSize: i32){
        self.fileSize = nfileSize;
    }

    pub fn getNoa(&self) -> i32{
        return self.noa.clone();
    }

    pub fn setNoa(&mut self, nnoa: i32){
        self.noa = nnoa;
    }

    pub fn getNod(&self) -> i32{
        return self.nod.clone();
    }

    pub fn setNod(&mut self, nnod: i32){
        self.nod = nnod;
    }

    pub fn getWhose(&self) -> String{
        return self.whose.clone();
    }

    pub fn setWhose(&mut self, nwhose: String){
        self.whose = nwhose;
    }

    pub fn getFileID(&self) -> i32{
        return self.fileId.clone();
    }

    pub fn setFileID(&mut self, nfileID: i32){
        self.fileId = nfileID;
    }


    pub fn getAllocateDeviceList(&mut self, query: &Query, nod: i32, noa: i32, whose: String) -> Vec<DeviceItem> {
        //确认有在线设备
        let mut onlineDevice = query.queryOnlineDevice();
        if(onlineDevice.len() == 0){
            let file = DeviceItem {
                id: 0,
                ip: "".to_string(),
                port: 0,
                is_online: false,
                rs: 0,
                time: 0,
                leftrs: 0,
            };
            return vec![file];
        }
        //计算相似度 0<=distance<=24
        let mut onlineDeviceNum = onlineDevice.len();
        let mut distance: Vec<i32> = Vec::new();
        for i in 0..onlineDeviceNum{
            let mut save = query.query_user_time(self.whose.clone());
            let mut time = onlineDevice[i].get_time();
            distance.push(0);
            for j in 0..24{
                if ((time & 1) == 0 && (save & 1) == 1){//0和1的&改为了&
                    distance[i] = distance[i] + 1;
                }
                time = time >> 1;
                save = save >> 1;
            }
        }

        let mut fragmentSize = self.fileSize / self.nod;
        // 由于有 vlab，必然有至少一台distance <= 30% * 24 = 7??
        //Java:ArrayList 类是一个可以动态修改的数组，与普通数组的区别就是它是没有固定大小的限制，我们可以添加或删除元素。
        let mut distanceId: Vec<usize> = Vec::new();
        //原本的java代码总是插入i到ArrayList的首部，这里采用反向的循环，总是插在vector的尾部，正确性有待验证?
        let mut i: usize = onlineDeviceNum - 1;
        println!("!!!");
        while i >= 0 {
            println!("onlineDevice[i].get_leftrs(): {0}, fragmantsize: {1}", onlineDevice[i].get_leftrs(), fragmentSize);
            if onlineDevice[i].get_leftrs() > fragmentSize {
             //if ((distance[i] <= 7) && (onlineDevice[i].get_leftrs() > fragmentSize)){
                // 差距够小 且 至少可以分配一个碎片
                distanceId.push(i.clone());
                
            }
            if i == 0 {
                break;
            }
            i = i - 1;
        }
        println!("111");
        let mut size = distanceId.len();// 有效在线主机数
        println!("online devices(size): {}", size);
        if size < 1 {
            let file = DeviceItem {
                id: 0,
                ip: "".to_string(),
                port: 0,
                is_online: false,
                rs: 0,
                time: 0,
                leftrs: 0,
            };
            return vec![file];
        }
        // 根据碎片数量和有效在线主机数，确定结果
        let mut deviceItemList: Vec<DeviceItem> = Vec::new();//原本初始化大小应为nod+noa/
        /*
        if(self.noa + self.nod <= (size as i32)) {
            for i in 0..self.nod + self.noa{
                deviceItemList.push(onlineDevice[distanceId[i as usize]].clone());
                let mut rs_size:i32 = deviceItemList[i as usize].get_leftrs() - fragmentSize;
                deviceItemList[i as usize].set_leftrs(rs_size);
            }
        }
        else{*/
        let mut i = self.noa + self.nod - 1;
        let mut j = 0;
        while i >= 0 {
            let mut thisdevice = onlineDevice[distanceId[j as usize]].clone();
            if thisdevice.get_leftrs() > fragmentSize {
                let mut rs_size:i32 =thisdevice.get_leftrs() - fragmentSize;
                thisdevice.set_leftrs(rs_size);
                deviceItemList.push(thisdevice.clone());
                //println!("thisdevice.get_leftrs(): {}", thisdevice.get_leftrs());
                query.alterDevice(thisdevice);
                i = i - 1;
            }
            j = (j + 1) % size;
        }
        
        return deviceItemList;
    }

    pub fn uploadRegister(&mut self) -> String{
        println!("uploadRegister is called");


        let mut query = Query::new();
        let qpath: Option<String> = Some(self.path.clone());
        let qname: Option<String> = Some(self.fileName.clone());
        let file_item = query.queryFile_Bypathname(qpath, qname);
        let mut online_device = query.queryOnlineDevice();

        //源代码部分都返回success，有点迷惑
        if online_device.len() == 0 {
            println!("1");
            self.result = String::from("NoOnlineDevices");
            let return_val = String::from("success");
            return return_val;
        }

        //nod:Number of division  noa:Number of append
        //fileItem != null ,考虑到query.queryFile_Bypathname的出错时的返回值，id=0或-1，此时认为没查询到
        if file_item.get_id() > 0 {
            self.result = String::from("DuplicateFileName");
            let return_val = String::from("success");
            return return_val;
        }
        else{
            let mut newFile = FileItem::init_2(self.fileName.clone(), self.path.clone(), "rwxrwxrwx".to_string(), "".to_string(),  self.noa.clone(), self.nod.clone(),false, self.fileType.clone(), self.fileSize.clone(), self.fileblocks.clone(),self.whose.clone());
            self.fileId = query.addFile(newFile);
            if self.fileId < 0{
                //TODO
            }
            let mut deviceID: i32;
            let mut str = String::new();
            //Java中的JSONArray:由JSONObject组成的数组
            let mut jsonArray:Vec<Value> = Vec::new();
            //let mut fileUploader = FileUploader::new();
            //下面提示出现了所有权问题,故加了clone
            let mut deviceItemList:Vec<DeviceItem> = self.getAllocateDeviceList(&query, self.nod.clone(), self.noa.clone(), self.whose.clone());
            //println!("in uploadreg: {}", deviceItemList[0].port);
            if deviceItemList[0].get_ip()==""{
                self.result = String::from("NotEnoughDevices");
                let mut return_val = String::from("success");
                return return_val;
            }
            for i in 0..self.noa + self.nod {
                let mut formDetailsJson = json!({
                    "filename": (self.fileId * 100 + i).to_string(),
                    "fragmentID": i.clone(),
                    "ip": deviceItemList[i as usize].get_ip(),
                    "port": deviceItemList[i as usize].get_port(),
                });
                jsonArray.push(formDetailsJson);
                query.addFragment(self.fileId * 100 + i, deviceItemList[i as usize].get_id().to_string());
            }
            if jsonArray.len() <((self.noa + self.nod)).try_into().unwrap() {//??
                self.result = String::from("NotEnoughDevices1");
                let mut return_val = String::from("success");
                return return_val;
            }
            else{
                //self.devices = serde_json::from_str("");
                self.devices = json!({
                    "forms": jsonArray,
                });
                println!("{}", self.devices.to_string());

                self.result = String::from("OK");
                let mut return_val = String::from("success");
                return return_val;
            }
        }
    }

    /*
    pub fn progressCheck(path1:String, name1:String) -> String{
        //return -1 if error
        //else, return a number from 0 to 100 as # of fragments which have been downloaded
        //let mut return_val = String::new();
        let fragmentFolderPath=PathBuf::from("/usr/local/tomcat/webapps/DFS/CloudDriveServer/downloadFragment");
        //let fileFolderPath=PathBuf::from("D:webapps/DFS/CloudDriveServer/tmpFile/");

        let query = Query::new();
        let qpath: Option<String> = Some(path1);
        let qname: Option<String> = Some(name1);
        let file_item = query.queryFile_Bypathname(qpath, qname);

        let file_id = file_item.get_id().to_string();
        if file_id == "-1" {
            let result = String::from("Error");
            return result;
        }
        else{
            let mut collected_files: i32 = 0;
            for entry in fragmentFolderPath.read_dir().unwrap(){
                let path = entry.unwrap().path();
                let str = path.file_name();
                match str {
                    None => continue,
                    Some(str) => {
                        let mut name = str.to_os_string().into_string().unwrap();
                        name.pop();
                        name.pop();
                        if name == file_id {
                            collected_files = collected_files + 1;
                        }
                    }
                }
            }
            let t1 = collected_files as f64;
            let t2 = file_item.get_noa() as f64;
            let percentage: f64 = 2.0 * t1 / t2;
            collected_files = (percentage * 100.0) as i32;
            println!("pregress check is called, return {}", collected_files);

            let result = collected_files.to_string();
            result
        }
    }

    pub fn decodeFile(path1:String, name1:String) -> String {
        //return 1 and DELETE ALL FRAGMENTS OF INPUT FILE if decode successfully
        //else, return 0
        let fragmentFolderPath=PathBuf::from("/usr/local/tomcat/webapps/DFS/CloudDriveServer/downloadFragment");
        let fileFolderPath=PathBuf::from("/usr/local/tomcat/webapps/DFS/CloudDriveServer/tmpFile");
        println!("decodeFile is called");
        let query = Query::new();
        let qpath: Option<String> = Some(path1.clone());
        let qname: Option<String> = Some(name1.clone());
        let file_item = query.queryFile_Bypathname(qpath, qname);

        //com.backblaze.erasure.Decoder.decode()
        //decode(shardsFolder:PathBuf,fileFolder:PathBuf,fid:i32,noa:i32) -> bool
        let file_id = file_item.get_id().to_string();
        //let mut str = String::new();
        let file_folder = fileFolderPath.join(name1);
        //println!("{}", file_folder.display());
        if Decoder::decode(fragmentFolderPath.clone(), file_folder, file_item.get_id(), file_item.get_noa()) {
            for entry in fragmentFolderPath.read_dir().unwrap(){
                let path = entry.unwrap().path();
                let str = path.file_name();
                match str {
                    None => continue,
                    Some(str) => {
                        let mut name = str.to_os_string().into_string().unwrap();
                        name.pop();
                        name.pop();
                        if name == file_id {
                            remove_file(path.as_path());
                        }
                    }
                }
            }

            let result = String::from("OK");
            //return_val = String::form("success");
            return result;
        }
        else {
            let result = String::from("Error");
            //return_val = String::form("success");
            return result;
        }
    }
    */


}