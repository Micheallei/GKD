pub struct DeviceItem {
    pub id: i32,
    pub ip: String,
    pub port: i32,
    pub is_online: bool,
    pub rs: i32,
}

impl DeviceItem {
    pub fn init(id: i32, ip: String, port: i32, is_online: bool, rs: i32) -> Self{
        DeviceItem {
            id: id,
            ip: ip,
            port: port,
            is_online: is_online,
            rs: rs,
        }
    }

    pub fn get_id(&mut self) -> i32 {
        self.id
    }

    pub fn get_ip(&mut self) -> String {
        let chars: Vec<char> = self.ip.chars().collect();
        let mut string = String::new();
        //string.push('"');
        for c in chars {
            string.push(c);
        }
        //string.push('"');
        //println!("string:{}",string);
        string //返回有问题
        
    }

    pub fn set_ip(&mut self, ip: String) {
        self.ip = ip;
    }

    pub fn get_port(&mut self) -> i32 {
        self.port
    }

    pub fn set_port(&mut self, port:i32) {
        self.port = port;
    }

    pub fn is_online(&mut self) -> bool {
        self.is_online
    }

    pub fn set_is_online(&mut self, is_online:bool) {
        self.is_online = is_online;
    }

    pub fn get_rs(&mut self) -> i32 {
        self.rs
    }

    pub fn set_rs(&mut self, rs: i32) {
        self.rs = rs;
    }
}