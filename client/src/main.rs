use std::io::Write;
use std::net::TcpStream;

const ADDRESS: &str = "127.0.0.1:1111";

fn main() {
    let mut stream = TcpStream::connect(ADDRESS).expect("WTF: Couldn't connect");

    stream.write("Hello server".as_bytes())
          .expect("WTF: Couldn't send");

}

