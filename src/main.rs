use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncBufReadExt, BufReader, AsyncWriteExt}, 
    sync::broadcast
};
use serde_json::Value;

use serde::{
    Serialize, Deserialize
};

use std::env::args;

#[derive(Serialize, Deserialize)]
struct MessageStructure {
    content: String,
    toaddr: String
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

                        let stream = TcpStream::connect(receiveaddr).await;

                        let mut stream = match stream {
                            Ok(stream) => stream,
                            Err(_) => break // now break but saving message to file is required.
                        };

                        let streamjson = serde_json::to_string(&msgstruct);

                        stream.write_all(streamjson.unwrap().as_bytes()).await.unwrap();

                        
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

fn value_to_msg_struct(value: Value) -> MessageStructure {
    let toaddr = &value["toaddr"];
    let content = &value["content"];

    let messagestruct = MessageStructure{
        content: content.to_string(),
        toaddr: toaddr.to_string()
    };

    return messagestruct
}


#[tokio::main]
async fn main() {
    let port = args().nth(1).expect("does not have required args: port");
    println!("Try run server on {}", port);
    let listener = TcpListener::bind(format!("0.0.0.0:{}",port)).await.unwrap();
    message_system(listener).await;
}