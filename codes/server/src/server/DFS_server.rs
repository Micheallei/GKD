use std::thread;

pub struct DFS_server {
    connecting: bool,
}

impl DFS_server {
    pub fn main (args: String) {
        println!("Server start");
        let con_port:String = String::from("127.0.0.1:6666");
        let con_t = super::controlConnect::ServerThread::ServerThread::new(con_port);
        thread::spawn( move ||{
            con_t.run();
        });
        let data_port:String = String::from("127.0.0.1:6668");
        let data_t = super::dataConnect::ServerThread::ServerThread::new(data_port);
        thread::spawn( move ||{
            data_t.run();
        });
        while true {

        }
        println!("DFS_server end");
    }
}