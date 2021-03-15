pub struct AnotherRequestItem {
    ip: String,
    port: String,
    file_name: String,
    file_type_: String,
    file_size: i32,
    fragment_id: i32,
}

impl AnotherRequestItem {
    fn init(ip: String,
        port: String,
        file_name: String,
        file_type_: String,
        file_size: i32,
        fragment_id: i32) -> Self {
        AnotherRequestItem {
            ip,
            port,
            file_name,
            file_type_,
            file_size,
            fragment_id,
        }
    }

    pub fn get_ip(&mut self) -> String {
        self.ip.clone()
    }

    pub fn get_file_type_(&mut self) -> String {
        self.file_type_.clone()
    }

    pub fn get_fragment_id(&mut self) -> i32 {
        self.fragment_id
    }

    pub fn get_port(&self) -> String {
        let chars: Vec<char> = self.port.chars().collect();
        let mut string = String::new();
        for c in chars {
            string.push(c);
        }
        string
    }

    pub fn set_port(&mut self, port: String) {
        self.port = port;
    }

    pub fn get_file_name(&self) -> String {
        let chars: Vec<char> = self.file_name.chars().collect();
        let mut string = String::new();
        for c in chars {
            string.push(c);
        }
        string
    }

    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }
}
