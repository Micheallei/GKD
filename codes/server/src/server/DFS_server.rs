use std::thread;
use super::database::DeviceItem::DeviceItem;
use log::{info,warn,debug,error,trace};
use log4rs;

pub struct DFS_server {
    connecting: bool,
}

impl DFS_server {
    pub fn main (args: String) {
        println!("Server start");
        info!("server start");
        let query = super::database::Query::Query::new();
        let mut devices:Vec<DeviceItem> = query.queryOnlineDevice();
        if !devices.is_empty() {
            for device in &mut devices {
                let dvc: DeviceItem = DeviceItem::init(device.get_id(), device.get_ip(), device.get_port(),
                                                        false, device.get_rs(),device.get_time(),device.get_leftrs());
                query.alterDevice(dvc);
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
        loop {

        }
        println!("DFS_server end");
    }
}