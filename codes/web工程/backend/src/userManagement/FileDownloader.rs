use std::path::PathBuf;
use std::path::Path;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, DirEntry, remove_file};
use serde_json::{Result, Value,json};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use super::super::database::Query::Query;
use super::super::com::Decoder::Decoder;
use super::super::database::RequestItem::RequestItem;
use super::super::database::AnotherRequestItem::AnotherRequestItem;

#[derive(Serialize, Deserialize)]
pub struct FileDownloader{
    path: String,
    name: String,
    result: String,
    devices:Value,
    fileType:String,
    fileSize:i64,
    noa:i64,
    nod:i64,//以上为返回值
    serialVersionUID: i64,
    //fragmentFolderPath1: PathBuf,
    //fileFolderPath1: PathBuf,
}

impl FileDownloader {
    pub fn new() -> FileDownloader {
        FileDownloader {
            path: String::new(),
            name: String::new(),
            result: String::new(),
            devices:json!({}),
            fileType:String::new(),
            fileSize:0,
            noa:0,
            nod:0,
            serialVersionUID: 1,
            //fragmentFolderPath1:PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/downloadFragment/"),
            //fileFolderPath1:PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/tmpFile/"),
        }
    }

    pub fn getPath(&self) -> String {
        self.path.clone()
    }

    pub fn setPath(&mut self, npath: String) {
        self.path = npath;
    }

    pub fn getResult(&self) -> String {
        self.result.clone()
    }

    pub fn setResult(&mut self, nresult: String) {
        self.result = nresult;
    }

    pub fn getName(&self) -> String {
        self.name.clone()
    }

    pub fn setName(&mut self, nname: String) {
        self.name = nname;
    }

    pub fn getDevices(&self) -> Value {
        self.devices.clone()
    }

    pub fn setDevices(&mut self, ndevices: Value) {
        self.devices = ndevices;
    }

    pub fn getFileType(&self) -> String {
        self.fileType.clone()
    }

    pub fn setFileType(&mut self, nfileType: String) {
        self.fileType = nfileType;
    }

    pub fn getFileSize(&self) -> i64 {
        self.fileSize.clone()
    }

    pub fn setFileSize(&mut self, nfileSize: i64) {
        self.fileSize = nfileSize;
    }

    pub fn getNoa(&self) -> i64 {
        self.noa.clone()
    }

    pub fn setNoa(&mut self, nnoa: i64) {
        self.noa = nnoa;
    }

    pub fn getNod(&self) -> i64 {
        self.nod.clone()
    }

    pub fn setNod(&mut self, nnod: i64) {
        self.nod = nnod;
    }

    pub fn downloadRegister(path1:String, name1:String) -> FileDownloader {
        //return -1 if error
		//return 0 if can not collect enough fragments
        //else, return 1
        println!("downloadRegister is called");

        //let mut return_val = String::new();
        let query = Query::new();
        let qpath: Option<String> = Some(path1);
        let qname: Option<String> = Some(name1);
        let mut file_item = query.queryFile_Bypathname(qpath, qname);
        let mut online_device = query.queryOnlineDevice();
        println!("name: {0}, path: {1}", file_item.get_name(),file_item.get_path());
        let mut filedownloader=FileDownloader::new();//要返回到main中的数据

        if online_device.len() == 0 {
            let result = String::from("NotEnoughFragments");
            //return_val = String::form("success");
            //return return_val;
            filedownloader.setResult(result);
            return filedownloader;
        }

        if (file_item.get_nod() < 1) {
            //query.closeConnection();
            let result = String::from("Error");
            //return_val = String::form("success");
            //return return_val;
            filedownloader.setResult(result);
            return filedownloader;
        }
        else {
            let nod = file_item.get_nod();
            let noa = file_item.get_noa();
            let id = file_item.get_id();
            //let mut str = String::new();
            let mut request_items: Vec<AnotherRequestItem> = Vec::new();
            let mut jsonArray: Vec<Value> = Vec::new();
            for i in 0..(noa+nod) {
                let str = query.query_fragment(id * 100 + i);
                //println!("query frgment result: {}", str);
                if str == "" || str == "-1" {
                    continue;
                }
                let device_id: i32 = str.parse().unwrap();
                for j in 0..online_device.len() {
                    if online_device[j].get_id() == device_id {
                        
                        let mut curDevice=query.queryDevice(device_id);
                        println!("query device: ip: {0}, port:{1}", curDevice.ip, curDevice.port);
                        //request_items.push(RequestItem::init_2(1, id*100 + i, device_id));//pqz,1改为i
                        let formDetailsJson = json!({
                            "filename": (id*100 + i).to_string(),
                            "fragmentId": i.clone(),
                            "ip":curDevice.get_ip(),
                            "port":curDevice.get_port().to_string()
                        });
                        jsonArray.push(formDetailsJson);
                        break;
                    }
                }
            }
            /*
            let temp = (noa / 2) as usize;
            if request_items.len() < temp {
                let result = String::from("NotEnoughFragments");
                //return_val = String::form("success");
                //return return_val;
                return result;
            }*/
            if jsonArray.len()<nod.try_into().unwrap() {
                //query.closeConnection();
                let result = String::from("NotEnoughFragments");
                filedownloader.setResult(result);
                return filedownloader;
            }
            else {
                /*
                for i in 0..temp {
                    query.addRequest(request_items[i].clone());
                }*/
                filedownloader.devices=json!({
                    "forms":jsonArray
                });
                filedownloader.setFileSize(file_item.get_file_size().into());
                filedownloader.setFileType(file_item.get_file_type());
                filedownloader.setNod(file_item.get_nod().into());
                filedownloader.setNoa(file_item.get_noa().into());
                filedownloader.setName(file_item.get_name());

                //query.closeConnection();
                let result = String::from("OK");
                filedownloader.setResult(result);
                //return_val = String::form("success");
                //return return_val;
                return filedownloader;
            }
        }
    }
    /*
    pub fn progressCheck(path1:String, name1:String) -> String{
        //return -1 if error
		//else, return a number from 0 to 100 as # of fragments which have been downloaded
        //let mut return_val = String::new();
        let fragmentFolderPath=PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/downloadFragment/");
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
        let fragmentFolderPath=PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/downloadFragment/");
        let fileFolderPath=PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/tmpFile/");
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
