use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:8081").unwrap();
    let hello = "hello world";
    println!("Hello, world!");
    stream.write(hello.as_bytes()).unwrap();
    let mut ret = [0; 20];

    stream.read(&mut ret).unwrap();
    println!("received response from server:{:?}", str::from_utf8(&ret).unwrap());
}
