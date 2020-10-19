use std::path::PathBuf;
use std::path::Path;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File, DirEntry, remove_file};

use super::super::database::Query::Query;
use super::super::com::Decoder::Decoder;
use super::super::database::RequestItem::RequestItem;

pub struct FileDownloader{
    path: String,
    name: String,
    result: String,
    serialVersionUID: i64,
    fragmentFolderPath1: PathBuf,
    fileFolderPath1: PathBuf,
}

impl FileDownloader {
    pub fn new() -> FileDownloader {
        FileDownloader {
            path: String::new(),
            name: String::new(),
            result: String::new(),
            serialVersionUID: 1,
            fragmentFolderPath1:PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/downloadFragment/"),
            fileFolderPath1:PathBuf::from("E:/Tomcat 9.0/webapps/DFS/CloudDriveServer/tmpFile/"),
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

    pub fn downloadRegister(path1:String, name1:String) -> String {
        //return -1 if error
		//return 0 if can not collect enough fragments
        //else, return 1
        println!("downloadRegister is called");

        //let mut return_val = String::new();
        let query = Query::new();
        let qpath: Option<String> = Some(path1);
        let qname: Option<String> = Some(name1);
        let file_item = query.queryFile_Bypathname(qpath, qname);
        let mut online_device = query.queryOnlineDevice();

        if online_device.len() == 0 {
            let result = String::from("NotEnoughFragments");
            //return_val = String::form("success");
            //return return_val;
            return result;
        }

        if file_item.get_noa() < 1 {
            let result = String::from("Error");
            //return_val = String::form("success");
            //return return_val;
            return result;
        }
        else {
            let noa = file_item.get_noa();
            let id = file_item.get_id();
            //let mut str = String::new();
            let mut request_items: Vec<RequestItem> = Vec::new();
            for i in 0..noa {
                let str = query.queryFragment(id * 100 + i);
                if str == "" || str == "-1" {
                    continue;
                }
                let device_id: i32 = str.parse().unwrap();
                for j in 0..online_device.len() {
                    if online_device[j].get_id() == device_id {
                        request_items.push(RequestItem::init_2(1, id*100 + i, device_id));//pqz,1改为i
                        break;
                    }
                }
            }
            let temp = (noa / 2) as usize;
            if request_items.len() < temp {
                let result = String::from("NotEnoughFragments");
                //return_val = String::form("success");
                //return return_val;
                return result;
            }
            else {
                for i in 0..temp {
                    query.addRequest(request_items[i].clone());
                }
                let result = String::from("OK");
                //return_val = String::form("success");
                //return return_val;
                return result;
            }
        }
    }

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

}
