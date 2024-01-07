use std::net::TcpListener;
// use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let listner: TcpListener = TcpListener::bind("localhost:8081").unwrap();

    println!("server listening on 8081");
    for stream in listner.incoming() {
        let mut stream = stream.unwrap();
        println!("{}", "connection established");
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        stream.write(&mut buffer).unwrap();
    }
}

