pub struct FileItem {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub attribute: String,
    pub time: String,
    pub noa: i32,
    pub is_folder: bool,
}

impl FileItem {
    fn init(id: i32, name: String, path: String, attribute: String,
        time: String, noa: i32, is_folder: bool) -> Self {
            FileItem {
                id,
                name,
                path,
                attribute,
                time,
                noa,
                is_folder,
            }
        }

    pub fn init_2(name: String, path: String, attribute: String,
        time: String, noa: i32, is_folder: bool) -> Self {
            FileItem {
                id: 0,
                name,
                path,
                attribute,
                time,
                noa,
                is_folder,
            }
        }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> String {
        let chars: Vec<char> = self.name.chars().collect();
        let mut string = String::new();
        for c in chars {
            string.push(c);
        }
        string
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_path(&mut self) -> String {
        let chars: Vec<char> = self.path.chars().collect();
        let mut string = String::new();
        for c in chars {
            string.push(c);
        }
        string
    }

    pub fn set_path(&mut self, path:String) {
        self.path = path;
    }

    pub fn get_attribute(&self) -> String {
        let chars: Vec<char> = self.attribute.chars().collect();
        let mut string = String::new();
        for c in chars {
            string.push(c);
        }
        string
    }

    pub fn set_attribute(&mut self, attribute: String) {
        self.attribute = attribute;
    }

    pub fn get_time(&self) -> String {
        let chars: Vec<char> = self.time.chars().collect();
        let mut string = String::new();
        for c in chars {
            string.push(c);
        }
        string
    }

    pub fn set_time(&mut self, time:String) {
        self.time = time;
    }

    pub fn get_noa(&self) -> i32{
        self.noa
    }

    pub fn set_noa(&mut self, noa:i32) {
        self.noa = noa;
    }

    pub fn is_folder(&mut self) -> bool {
        self.is_folder
    }

    pub fn set_folder(&mut self, is_folder:bool) {
        self.is_folder = is_folder;
    }
}