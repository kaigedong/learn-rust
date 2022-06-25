use std::io::{Read, Write}; // TcpStream 实现了io::Read, io::Write这两个Trait
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    // 只监听一次请求:
    // let result = listener.accept().unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];
        stream.read_exact(&mut buffer).unwrap();
        stream.write_all(&buffer).unwrap();
    }
}
