mod datatypes;
mod storage;
mod collections;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    fs::{File},
};

use storage::Storage;

use crate::collections::Storage::HashMap as HashMap;



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
    let mut vault = File::open("dbmini.json").unwrap();
    let mut data = Storage::new();
    
    let PING: String = String::from("PING");
    let SET: String = String::from("SET");
    while let Ok(r) = stream.read(&mut buf){
        let cleaned_com = String::from_utf8_lossy(&buf[0..r]).to_string();
        match  cleaned_com {
            PING => {stream.write(b"+PONG\r\n").unwrap();},
            SET => {println!("at set")},
            GET => {}
            _ => unimplemented!(),
        };
    };

}

fn parsing() {
    /*
    commands
    GET <key>
    SET <key> <value>
    DELETE <key>
    FLUSH
    MGET <key1> ... <keyn>
    MSET <key1> <value1> ... <keyn> <valuen>
    */
}
