#[macro_use]
extern crate mysql;
mod server;
fn main() {
    println!("Hello, world!");
    crate::server::DFS_server::DFS_server::main("args".to_string());
}
