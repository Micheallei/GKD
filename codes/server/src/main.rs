use log::{info,warn,debug,error,trace};
use log4rs;

#[macro_use]
extern crate mysql;
mod server;
fn main() {
    println!("Hello, world!");
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    crate::server::DFS_server::DFS_server::main("args".to_string());
}
