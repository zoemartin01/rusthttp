use std::io::{BufRead, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

use itertools::Itertools;
use request::Request;

mod request;

fn listen(port: u16) {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(_) => panic!("Could not bind to port {port}"),
        };

        handle_conn(stream);

        println!("Connection established!");
    }
}

fn handle_conn(mut stream: TcpStream) {
    let request = Request::from_stream(&stream);

    println!("{:#?}", from_utf8(&request.body));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    listen(8080)
}
