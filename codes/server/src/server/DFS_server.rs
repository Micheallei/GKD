use std::thread;

pub struct DFS_server {
    connecting: bool,
}

impl DFS_server {
    pub fn main (args: String) {
        println!("Server start");
        let query = Query::new();
        let devices:Vec<DeviceItem> = query.queryOnlineDevice();
        if !devices.is_empty() {
            for device in &mut devices {
                device.set_is_online(false);
                query.alterDevice(device);
            }
        }
        let con_port:String = String::from("127.0.0.1:6666");   //设置controlPort
        let con_t = super::controlConnect::ServerThread::ServerThread::new(con_port);
        thread::spawn( move ||{
            con_t.run();
        });

        /*dontpanic删除
        let data_port:String = String::from("127.0.0.1:6668");
        let data_t = super::dataConnect::ServerThread::ServerThread::new(data_port);
        thread::spawn( move ||{
            data_t.run();
        });
        */
        while true {

        }
        println!("DFS_server end");
    }
}