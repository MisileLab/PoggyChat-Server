use tokio::{
    net::TcpListener,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}
};

use std::env::{
    args
};

#[tokio::main]
async fn main() {
    let port = args().nth(1).expect("does not have required args: port");
    println!("Try run server on {}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
    // Get message from client -> Save message history in file -> Send message to other client
}

async fn message_system(listener: TcpListener) {
    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (readsocket, mut writesocket) = socket.split();
            let mut reader = BufReader::new(readsocket);
            let mut line = String::new();
            
            loop {
                let read_bytes = reader.read_line(&mut line).await.unwrap();
                if read_bytes < 1 {
                    break;
                }
                writesocket.write_all(&line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
