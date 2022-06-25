use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    // TcpStream只能读写原始字节
    // .write返回 written amount，为 Result<usize> 类型
    // .wirte不保证能写完所有的bytes。使用write_all更好一点
    stream.write_all("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];
    stream.read_exact(&mut buffer).unwrap();
    println!(
        "Response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
