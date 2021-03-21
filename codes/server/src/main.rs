#[macro_use]
extern crate mysql;
use log::info;
use log4rs;
mod server;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    crate::server::DFS_server::DFS_server::main("args".to_string());
}
