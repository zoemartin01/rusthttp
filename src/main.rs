use std::io::Write;
use std::net::{TcpListener, TcpStream};

use request::Request;

use crate::response::Response;

mod request;
mod response;

fn listen(port: u16) {
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(_) => panic!("Could not bind to port {port}"),
        };

        handle_conn(stream);
    }
}

fn send(response: Response, mut tcp_stream: &TcpStream) {
    tcp_stream
        .write_all(response.as_bytes().as_slice())
        .unwrap();
}

fn handle_conn(stream: TcpStream) {
    let request = Request::from_stream(&stream);

    println!("{:#?}", request);

    let mut response = Response::default();
    response.status = 201;

    send(response, &stream);
}

fn main() {
    listen(8080)
}
