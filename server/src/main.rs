use std::io::Read;
use std::net::{TcpListener, TcpStream};

const ADDRESS: &str = "127.0.0.1:1111";

fn client_handler(mut stream: TcpStream) {
    let mut buffer = [0 as u8; 50];
    while stream.read(&mut buffer).is_err() {
    }

    let text = String::from_utf8_lossy(&buffer);
    println!("[Client]: {}", text);
}

fn main() {
    let listener = TcpListener::bind(ADDRESS).expect("WTF: Couldn't bind");
    println!("Good server");

    for stream in listener.incoming() {
        if stream.is_ok() {
            client_handler(stream.unwrap());
        }
    }
}

