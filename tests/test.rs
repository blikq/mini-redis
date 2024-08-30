use std::{
    net::{TcpStream, TcpListener},
    io::{Read, Write},
    fs,
    str};



#[test]
fn test_server() {
    let mut connection = TcpStream::connect("127.0.0.1:6379").unwrap();
    let _ = connection.write(b"PING");
    let mut buf = [0; 512];
    let mut lc: usize = 0; 
    while let Ok(r) = connection.read(&mut buf){
        lc = r;
        break;
    }

    let expected_response = String::from("+PONG\r\n");
    let response = String::from_utf8_lossy(&buf[..lc]);
    assert_eq!(response, expected_response);
}

#[test]
fn test_set_endpoint() {
    let mut connection = TcpStream::connect("127.0.0.1:6379").unwrap();
    let _ = connection.write(b"SET");
    let mut buf = [0; 512];
    assert!(true);
}