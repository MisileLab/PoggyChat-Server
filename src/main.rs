use tokio::{
    net::TcpListener,
    io::{AsyncBufReadExt, BufReader}, sync::broadcast
};

use serde_json::{
    Value
};

use std::env::{
    args
};

#[tokio::main]
async fn main() {
    let port = args().nth(1).expect("does not have required args: port");
    println!("Try run server on {}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();
    message_system(listener).await;
    // Get message from client -> Save message history in file -> Send message to other client
}

async fn message_system(listener: TcpListener) {
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (readsocket, _writesocket) = socket.split();
            let mut reader = BufReader::new(readsocket);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() < 1 {
                            break;
                        }
                        
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }

                    result = rx.recv() => {
                        let (msg, receiveaddr) = result.unwrap();

                        let msg: &str = msg.as_str();
                        let value = msg_to_value(msg);
                        let msgstruct = value_to_msg_struct(value);

                        

                        
                    }
                }
            }
        });
    }
}

fn msg_to_value(msg: &str) -> Value {
    let v: Value = serde_json::from_str(msg).unwrap();

    return v;
}

fn value_to_msg_struct(value: Value) -> MessageStruct {
    let toaddr = &value["toaddr"];
    let content = &value["content"];

    let messagestruct = MessageStruct{
        content: content.to_string(),
        toaddr: toaddr.to_string(),
    };

    return messagestruct
}

struct MessageStruct {
    content: String,
    toaddr: String
}