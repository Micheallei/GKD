extern crate reed_solomon_erasure;
#[macro_use]


mod client;
use log::info;
use log4rs;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("INFO:first log information");
    crate::client::client::client::main();
}


//mod server;