use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::TcpListener;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 200];
            stream.read_exact(&mut read_buffer).unwrap();

            // 将浏览器发来的请求转换成自定义的HttpRequest类型
            // 类似：
            // GET /styles.css HTTP/1.1\r\n
            // Host: localhost:3000\r\n
            // User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:101.0) Gecko/20100101 Firefox/101.0\r\n
            // Accept: text/css,*/*;q=0.1\r\n
            // Accept-Language: zh-CN,zh;q=0.8,zh-TW;q=
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec()).unwrap().into();

            Router::route(req, &mut stream);
        }
    }
}
