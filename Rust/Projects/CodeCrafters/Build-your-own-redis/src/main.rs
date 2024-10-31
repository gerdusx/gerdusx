#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    println!("Waiting for incoming connections...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let mut buf = [0; 1024];
                stream.read(&mut buf).unwrap();

                let buf = b"+PONG\r\n";
                stream.write(buf).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
