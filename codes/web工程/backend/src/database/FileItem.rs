pub struct FileItem {
    id: i32,
    name: String,
    path: String,
    attribute: String,
    time: String,
    nod: i32,
    noa: i32,
    is_folder: bool,
    file_type: String,
    file_size: i32,
    whose: String,
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
                nod,
                noa,
                is_folder,
                file_type,
                file_size,
                whose,
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
                nod,
                noa,
                is_folder,
                file_type,
                file_size,
                whose,
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

    pub fn get_nod(&self) -> i32{
        self.nod
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

    pub fn get_file_type(&self) -> String {
        let chars: Vec<char> = self.file_type.chars().collect();
        let mut string = String::new();
        for c in chars {
            string.push(c);
        }
        string
    }

    pub fn get_file_size(&mut self) {
        self.file_size
    }

    pub fn set_whose(&mut self, whose:String) {
        self.whose = whose;
    }

    pub fn get_whose(&self) -> String {
        let chars: Vec<char> = self.whose.chars().collect();
        let mut string = String::new();
        for c in chars {
            string.push(c);
        }
        string
    }
}