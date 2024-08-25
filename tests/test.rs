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
    while let Ok(r) = connection.read(&mut buf){
        break;
    }
    let expected_response = b"+PONG\r\n";
    assert_eq!(buf[..7], *expected_response);
}
