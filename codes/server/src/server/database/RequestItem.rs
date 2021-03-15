pub struct RequestItem {
    pub id: i32,
    pub type_: i32,
    pub fragmentId: i32,
    pub deviceId: i32,
}

impl RequestItem {
    fn init(id: i32, type_: i32, fid: i32, did: i32) -> Self {
        RequestItem {
            id,
            type_,
            fragmentId: fid,
            deviceId: did,
        }
    }

    pub fn init_2(type_: i32, fid: i32, did: i32) -> Self {
        RequestItem {
            id: 0,
            type_,
            fragmentId: fid,
            deviceId: did,
        }
    }

    pub fn clone(&self) -> RequestItem {
        RequestItem {
            id: self.id,
            type_: self.type_,
            fragmentId: self.fragmentId,
            deviceId: self.deviceId,
        }
    }

    pub fn get_id(&mut self) -> i32 {
        self.id
    }

    pub fn get_type(&mut self) -> i32 {
        self.type_
    }

    pub fn set_type(&mut self, type_: i32) {
        self.type_ = type_;
    }

    pub fn get_fragment_id(&mut self) -> i32 {
        self.fragmentId
    }

    pub fn set_fragment_id(&mut self, fragmentId: i32) {
        self.fragmentId = fragmentId;
    }

    pub fn get_device_id(&mut self) -> i32 {
        self.deviceId
    }

    pub fn set_device_id(&mut self, id: i32) {
        self.deviceId = id;
    }
}