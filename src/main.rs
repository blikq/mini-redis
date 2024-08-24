// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};


fn handle_stream(mut stream: TcpStream){
    let mut buf = [0; 512];
    while let Ok(r) = stream.read(&mut buf){
        println!("{}", r);
        stream.write(b"+PONG\r\n").unwrap();
    }

}

#[tokio::main]
async fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                handle_stream(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }  

}
