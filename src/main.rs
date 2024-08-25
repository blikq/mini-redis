mod datatypes;
mod storage;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

use storage::Storage;

#[tokio::main]
async fn main() {
    
    let listener = TcpListener::bind("127.0.0.1:6379").expect("couldn't bind to address");
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

fn handle_stream(mut stream: TcpStream){
    let mut buf = [0; 512];

    let PING = String::from("PING");

    while let Ok(r) = stream.read(&mut buf){
        match buffer_to_string(&buf) {
            PING => {stream.write(b"+PONG\r\n").unwrap();},
            _ => unimplemented!(),
        };
    };

}









pub fn buffer_to_string(buffer: &[u8; 512]) -> String {
    // Find the first null byte (0)
    let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());

    // Convert the non-null part of the buffer to a &str
    let valid_slice = &buffer[..len];

    // Convert the slice to a String
    String::from_utf8_lossy(valid_slice).to_string()
}


fn start(){

}