/*
pub struct SynItem{
    pub status: i32,
}

impl SynItem{
    pub fn new(&mut self, s:i32){
        self.status = s;
    }

    pub fn getStatus(&self)->i32{
        self.status
    }

    pub fn setStatus(&mut self,s:i32){
        self.status = s;
        //notify 同步实现
    }

    pub fn waitChange(&self,oldValue:i32) -> i32{
        while self.status == oldValue{
        //同步实现
        }
    }
}
*/