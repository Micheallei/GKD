use std::*;
use std::convert::TryInto;


//include!("FileItem.rs");
//include!("RequestItem.rs");
//include!("DeviceItem.rs");
use super::FileItem::FileItem;
use super::DeviceItem::DeviceItem;
use super::RequestItem::RequestItem;
use mysql as my;

struct UserItem {
    id: i32,
    name: Option<String>,
    passwd: Option<String>,
}

struct FragmentItem {
    id: i32,
    path: Option<String>,
}

pub struct Query{
    pool: mysql::Pool
}

impl Query {
    pub fn new() -> Query{
        //需要大家在自己的电脑把 root:XXXX 改成自己的 mysql 密码
        let pool = my::Pool::new("mysql://root:pqzpqzpqz@localhost:3306/mysql").unwrap();
        Query {
            pool: pool,
        }
    }

    pub fn queryFile_Bypathname(&self, path: Option<String>, name: Option<String>) -> FileItem{
        let selected: Result<Vec<FileItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.file WHERE NAME = :name AND PATH = :path", 
                params!{"name" => name, "path" => path})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, path, attribute, time, noa, is_folder) = my::from_row(row);
                FileItem {
                    id: id,
                    name: name,
                    path: path,
                    attribute: attribute,
                    time: time,
                    noa: noa,
                    is_folder: is_folder,
                }
            }).collect()
        });
        match &selected {
            Err(e) => {
                return FileItem {
                    id: -1, 
                    name: "".to_string(), 
                    path: "".to_string(), 
                    attribute: "".to_string(),
                    time: "".to_string(),
                    noa: 0,
                    is_folder: false,
                }
            }
            Ok(selected_files) => {
                if selected_files.len() == 0 {
                    return FileItem {
                        id: 0, 
                        name: "".to_string(), 
                        path: "".to_string(), 
                        attribute: "".to_string(),
                        time: "".to_string(),
                        noa: 0,
                        is_folder: false,
                    }
                }
                return FileItem {
                    id: selected_files[0].id,
                    name: selected_files[0].name.clone(),
                    path: selected_files[0].path.clone(),
                    attribute: selected_files[0].attribute.clone(),
                    time: selected_files[0].time.clone(),
                    noa: selected_files[0].noa,
                    is_folder: selected_files[0].is_folder,
                }
            }
        }
    }

    pub fn queryFile_Byid(&self, id: i32) -> FileItem {
        let selected: Result<Vec<FileItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.file WHERE ID = :id", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, path, attribute, time, noa, is_folder) = my::from_row(row);
                FileItem {
                    id: id,
                    name: name,
                    path: path,
                    attribute: attribute,
                    time: time,
                    noa: noa,
                    is_folder: is_folder,
                }
            }).collect()
        });
        match &selected {
            Err(e) => {
                return FileItem {
                    id: -1, 
                    name: "".to_string(), 
                    path: "".to_string(), 
                    attribute: "".to_string(),
                    time: "".to_string(),
                    noa: 0,
                    is_folder: false,
                }
            }
            Ok(selected_files) => {
                if selected_files.len() == 0 {
                    return FileItem {
                        id: 0, 
                        name: "".to_string(), 
                        path: "".to_string(), 
                        attribute: "".to_string(),
                        time: "".to_string(),
                        noa: 0,
                        is_folder: false,
                    }
                }
                return FileItem {
                    id: selected_files[0].id,
                    name: selected_files[0].name.clone(),
                    path: selected_files[0].path.clone(),
                    attribute: selected_files[0].attribute.clone(),
                    time: selected_files[0].time.clone(),
                    noa: selected_files[0].noa,
                    is_folder: selected_files[0].is_folder,
                }
            }
        }
    }

    pub fn queryFile_Bypath(&self, path: Option<String>) -> Vec<FileItem>{
        let selected_files: Result<Vec<FileItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.file WHERE PATH = :path", 
                params!{"path" => path})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, path, attribute, time, noa, is_folder) = my::from_row(row);
                FileItem {
                    id: id,
                    name: name,
                    path: path,
                    attribute: attribute,
                    time: time,
                    noa: noa,
                    is_folder: is_folder,
                }
            }).collect()
        });
        if let Err(e) = selected_files {
            let file = FileItem {
                id: -1, 
                name: "".to_string(), 
                path: "".to_string(), 
                attribute: "".to_string(),
                time: "".to_string(),
                noa: 0,
                is_folder: false,
            };
            return vec![file];
        }
        let files = selected_files.unwrap();
        if files.len() == 0 {
            let file =  FileItem {
                id: 0, 
                name: "".to_string(), 
                path: "".to_string(), 
                attribute: "".to_string(),
                time: "".to_string(),
                noa: 0,
                is_folder: false,
            };
            return vec![file];
        }
        files
    }
    
    pub fn queryFragment(&self, id: i32) -> String {
        let selected_fragments: Result<Vec<FragmentItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.FRAGMENT WHERE ID = :id", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, path) = my::from_row(row);
                FragmentItem {
                    id: id,
                    path: path,
                }
            }).collect()
        });
        if let Err(e) = selected_fragments {
            return "".to_string();
        }
        let fragment = selected_fragments.unwrap();
        if fragment.len() == 0 {
            return "".to_string();
        }
        return fragment[0].path.clone().unwrap();
    }

    pub fn queryFragmentNumbers(&self, fileId: i32) -> i32{
        let selected_fragments: Result<Vec<FragmentItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.fragment WHERE ID>=:id_1 AND ID<:id_2", 
                params!{"id_1" => fileId*100, "id_2" => (fileId+1)*100})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, path) = my::from_row(row);
                FragmentItem {
                    id: id,
                    path: path,
                }
            }).collect()
        });
        if let Err(e) = selected_fragments {
            return -1;
        }
        let mut i: i32 = 0;
        for _f in selected_fragments.unwrap() {
            i = i+1;
        }
        i
    }
    //no FragmentItem

    pub fn queryOnlineDevice(&self) -> Vec<DeviceItem> {
        let selected_devices: Result<Vec<DeviceItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.DEVICE WHERE ISONLINE=true ORDER BY RS DESC", 
                ())
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, ip, port, is_online, rs) = my::from_row(row);
                DeviceItem {
                    id: id,
                    ip: ip,
                    port: port,
                    is_online: is_online,
                    rs: rs,
                }
            }).collect()
        });
        if let Err(e) = selected_devices {
            let file = DeviceItem {
                id: -1,
                ip: "".to_string(),
                port: 0,
                is_online: false,
                rs: 0,
            };
            return vec![file];
        }
        let devices = selected_devices.unwrap();
        if 0 == devices.len() {
            let file = DeviceItem {
                id: 0,
                ip: "".to_string(),
                port: 0,
                is_online: false,
                rs: 0,
            };
            return vec![file];
        }
        devices
    }

    pub fn queryDevice(&self, id: i32) -> DeviceItem {
        let selected_devices: Result<Vec<DeviceItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.DEVICE WHERE ID=:id", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, ip, port, is_online, rs) = my::from_row(row);
                DeviceItem {
                    id: id,
                    ip: ip,
                    port: port,
                    is_online: is_online,
                    rs: rs,
                }
            }).collect()
        });
        if let Err(e) = selected_devices {
            return DeviceItem {
                id: -1,
                ip: "".to_string(),
                port: 0,
                is_online: false,
                rs: 0,
            };
        }
        let devices = selected_devices.unwrap();
        return DeviceItem {
            id: devices[0].id,
            ip: devices[0].ip.clone(),
            port: devices[0].port.clone(),
            is_online: devices[0].is_online,
            rs: devices[0].rs,
        }
    }

    pub fn queryRequest_Byid(&self, id: i32) -> RequestItem {
        let selected_requests: Result<Vec<RequestItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.REQUEST WHERE ID=:id", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        });
        if let Err(e) = selected_requests {
            return RequestItem {
                id: -1,
                type_: 0,
                fragmentId: 0,
                deviceId: 0,
            };
        }
        let requests = selected_requests.unwrap();
        if 0 == requests.len() {
            return RequestItem {
                id: 0,
                type_: 0,
                fragmentId: 0,
                deviceId: 0,
            };
        }
        return RequestItem {
            id: requests[0].id,
            type_: requests[0].type_,
            fragmentId: requests[0].fragmentId,
            deviceId: requests[0].deviceId,
        }
    }

    pub fn queryFirstRequest_Byid(&self, id: i32) -> RequestItem {
        let selected_requests: Result<Vec<RequestItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.REQUEST WHERE DEVICEID=:id LIMIT 1", 
                params!{"id" => id})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        });
        if let Err(e) = selected_requests {
            return RequestItem {
                id: -1,
                type_: 0,
                fragmentId: 0,
                deviceId: 0,
            };
        }
        let requests = selected_requests.unwrap();
        if 0 == requests.len() {
            return RequestItem {
                id: 0,
                type_: 0,
                fragmentId: 0,
                deviceId: 0,
            };
        }
        return RequestItem {
            id: requests[0].id,
            type_: requests[0].type_,
            fragmentId: requests[0].fragmentId,
            deviceId: requests[0].deviceId,
        }
    }

    pub fn queryRequest_Bydeviceid(&self, deviceId: i32) -> RequestItem {
        let selected_requests: Result<Vec<RequestItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.REQUEST WHERE DEVICEID=:id", 
                params!{"id" => deviceId})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        });
        if let Err(e) = selected_requests {
            return RequestItem {
                id: -1,
                type_: 0,
                fragmentId: 0,
                deviceId: 0,
            };
        }
        let requests = selected_requests.unwrap();
        if 0 == requests.len() {
            return RequestItem {
                id: 0,
                type_: 0,
                fragmentId: 0,
                deviceId: 0,
            };
        }
        return RequestItem {
            id: requests[0].id,
            type_: requests[0].type_,
            fragmentId: requests[0].fragmentId,
            deviceId: requests[0].deviceId,
        }
    }

    pub fn queryRequestNumbers_Byid(&self, deviceId: i32) -> i32 {
        let selected_requests: Result<Vec<RequestItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.REQUEST WHERE DEVICEID=:id", 
                params!{"id" => deviceId})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        });
        if let Err(e) = selected_requests {
            return -1;
        }
        let mut i: i32 = 0;
        for _r in selected_requests.unwrap() {
            i = i+1;
        }
        i
    }

    pub fn queryRequestNumbers_Byidtype(&self, fileId: i32, type_: i32) -> i32 {
        let selected_requests: Result<Vec<RequestItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.REQUEST WHERE FRAGMENTID>=:fid
                AND FRAGMENTID<:fid2 AND TYPE_=:type_",
                params!{"fid" => fileId*100, "fid2" => (fileId+1)*100, "type_" => type_})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, type_, fragmentId, deviceId) = my::from_row(row);
                RequestItem {
                    id: id,
                    type_: type_,
                    fragmentId: fragmentId,
                    deviceId: deviceId,
                }
            }).collect()
        });
        if let Err(e) = selected_requests {
            return -1;
        }
        let mut i: i32 = 0;
        for _r in selected_requests.unwrap() {
            i = i+1;
        }
        i
    }

    pub fn queryUserPasswd(&self, name: Option<String>) -> Option<String> {
        let selected_user: Result<Vec<UserItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.USER WHERE NAME=:name",
                params!{"name" => &name})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, passwd) = my::from_row(row);
                UserItem {
                    id: id,
                    name: name,
                    passwd: passwd,
                }
            }).collect()
        });
        if let Err(e) = selected_user {
            return None;
        }
        let users = selected_user.unwrap();
        if 0 == users.len() {
            return Some(" ".to_string());
        }
        users[0].passwd.clone()
    }

    pub fn queryUserID(&self, name: Option<String>) -> i32 {
        let selected_user: Result<Vec<UserItem>, mysql::Error> =
        self.pool.prep_exec("SELECT * FROM DFS.USER WHERE NAME=:name",
                params!{"name" => &name})
        .map(|result| { 
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name, passwd) = my::from_row(row);
                UserItem {
                    id: id,
                    name: name,
                    passwd: passwd,
                }
            }).collect()
        });
        if let Err(e) = selected_user {
            return -1;
        }
        selected_user.unwrap()[0].id
    }
}

impl Query{
    pub fn addFile(&self, mut file:FileItem) -> i32{
        let mut suc:i32 = -1;
        if file.is_folder(){
            for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FILE (NAME,PATH,ATTRIBUTE,TIME,NOA,ISFOLDER) 
                VALUES (:name,:path,:attribute,:time,:noa,true);").into_iter() {
                suc = stmt.execute(params!{
                    "name" => file.get_name(),
                    "path" => file.get_path(),
                    "attribute" => file.get_attribute(),
                    "time" => file.get_time(),
                    "noa" => file.get_noa()
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        } else {
            for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FILE (NAME,PATH,ATTRIBUTE,TIME,NOA,ISFOLDER) 
                VALUES (:name,:path,:attribute,:time,:noa,false);").into_iter() {
                suc = stmt.execute(params!{
                    "name" => file.get_name(),
                    "path" => file.get_path(),
                    "attribute" => file.get_attribute(),
                    "time" => file.get_time(),
                    "noa" => file.get_noa()
                }).unwrap().last_insert_id().try_into().unwrap();
            //此处未处理execute不成功时，返回-1的情况
            }
        }
        return suc;
    }

    pub fn deleteFile(&self,id:i32) -> i32{
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.FILE WHERE ID=:id").into_iter() {
            stmt.execute(params!{
                "id" => id
            }).unwrap();
        //此处未处理execute不成功时，返回-1的情况
        }
        suc = 1;
        return suc;
    }

    pub fn alterFile(&self, mut file:FileItem) -> i32{
        let mut suc:i32 = -1;
        if file.is_folder(){
            for mut stmt in self.pool.prepare(r"UPDATE DFS.FILE SET NAME=:name,PATH=:path,ATTRIBUTE=:attribute,
            TIME=:time,NOA=:noa,ISFOLDER=true WHERE id=:id;").into_iter() {
                stmt.execute(params!{
                    "name" => file.get_name(),
                    "path" => file.get_path(),
                    "attribute" => file.get_attribute(),
                    "time" => file.get_time(),
                    "noa" => file.get_noa(),
                    "id" => file.get_id() 
                }).unwrap().last_insert_id() as i32;
            //此处未处理execute不成功时，返回-1的情况
            }
        } else {
            for mut stmt in self.pool.prepare(r"UPDATE DFS.FILE SET NAME=:name,PATH=:path,ATTRIBUTE=:attribute,
            TIME=:time,NOA=:noa,ISFOLDER=false WHERE id=:id;").into_iter() {
                stmt.execute(params!{
                    "name" => file.get_name(),
                    "path" => file.get_path(),
                    "attribute" => file.get_attribute(),
                    "time" => file.get_time(),
                    "noa" => file.get_noa(),
                    "id" => file.get_id() 
                }).unwrap().last_insert_id() as i32;
            //此处未处理execute不成功时，返回-1的情况
            }
        }
        suc = 1;
        return suc;
    }

    pub fn alterDevice(&self, mut device:DeviceItem) -> i32{
        //println!("enter alterDevice");//note:by lyf
        //println!("device:ip={},port={},rs={},id={}",device.get_ip(),device.get_port(),device.get_rs(),device.get_id());
        let mut suc:i32 = -1;
        if device.is_online(){
            for mut stmt in self.pool.prepare("UPDATE DFS.DEVICE SET IP=:ip,PORT=:port,ISONLINE=true,
            RS=:rs WHERE id=:id;").into_iter() {
                let res = stmt.execute(params!{
                    "ip" => device.get_ip(),
                    "port" => device.get_port(),
                    "rs" => device.get_rs(),
                    "id" => device.get_id()
                });
                suc = match res{
                    Ok(_) => 1,
                    Err(_) => -1
                };
            }
            
        } else {
            for mut stmt in self.pool.prepare(r"UPDATE DFS.DEVICE SET IP=:ip,PORT=:port,ISONLINE=false,
            RS=:rs WHERE id=:id;").into_iter() {
                let res = stmt.execute(params!{
                    "ip" => device.get_ip(),
                    "port" => device.get_port(),
                    "rs" => device.get_rs(),
                    "id" => device.get_id()
                });    
                suc = match res{
                    Ok(_) => 1,
                    Err(_) => -1
                };   
            }
            
        }       
        return suc;
    }

    pub fn addFragment(&self,id:i32, path:String) -> i32{
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.FRAGMENT VALUES (:id,:path)").into_iter() {
            let res = stmt.execute(params!{
                "id" => id,
                "path" => path.clone()
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    } 

    pub fn deleteFragment(&self,id:i32) -> i32{
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.FRAGMENT WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn alterFragment(&self,id:i32,path:String) -> i32{
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"UPDATE DFS.FRAGMENT SET PATH=:path WHERE id=:id;").into_iter() {
            let res = stmt.execute(params!{
                "path" => path.clone(),
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn addRequest(&self, mut request:RequestItem) -> i32{
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.REQUEST (TYPE,FRAGMENTID,DEVICEID)
        VALUES (:type,:fragmentid,:deviceid)").into_iter() {
            suc = stmt.execute(params!{
                "type" => request.get_type(),
                "fragmentid" => request.get_fragment_id(),
                "deviceid" => request.get_device_id()
            }).unwrap().last_insert_id().try_into().unwrap();
        //此处未处理execute不成功时，返回-1的情况
        }
        suc = 1;
        return suc;
    }

    pub fn deleteRequest(&self,id:i32) -> i32{
        println!("enter delete request");
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.REQUEST WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }
    pub fn addUser(&self,name:String,passwd:String)-> i32{
        //suc为INSERT的用户在mysql中的id
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"INSERT INTO DFS.USER (NAME,PASSWD) VALUES (:name, :passwd)").into_iter() {
            let res = stmt.execute(params!{
                "name" => name.clone(),
                "passwd" => passwd.clone()
            });//unwrap().last_insert_id().try_into().unwrap();
            suc = match res{
                Ok(_) => res.unwrap().last_insert_id().try_into().unwrap(),
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn alterUser(&self,id:i32,name:String,passwd:String) -> i32{
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"UPDATE INTO DFS.USER SET NAEME=:name,PASSWD=passwd WHERE id=:id").into_iter() {
            let res = stmt.execute(params!{
                "name" => name.clone(),
                "passwd" => passwd.clone(),
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }

    pub fn deleteUser(&self,id:i32) -> i32{
        let mut suc:i32 = -1;
        for mut stmt in self.pool.prepare(r"DELETE FROM DFS.USER WHERE ID=:id").into_iter() {
            let res = stmt.execute(params!{
                "id" => id
            });
            suc = match res{
                Ok(_) => 1,
                Err(_) => -1
            };
        }
        return suc;
    }
}
