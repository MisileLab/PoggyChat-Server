use std::net::{
    TcpListener,
    TcpStream
};

use std::env::{
    args
};

fn main() {
    let port = args().nth(1).expect("does not have required args: port");
    println!("Try run server on {}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port));
    // Get message from client -> Save message history in file -> Send message to other client
}
