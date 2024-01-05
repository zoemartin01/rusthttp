use std::fs;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::path::Path;

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

    let mut filepath = if request.path.ends_with("/") {
        request.path + "index.html"
    } else {
        request.path
    };

    let mut path = Path::new(&format!("static/{filepath}")).to_owned();

    if path.exists() && path.is_dir() {
        filepath = filepath + "/index.html";
        path = Path::new(&format!("static/{filepath}")).to_owned();
    }

    if path.exists() && path.is_file() {
        let content = fs::read_to_string(format!("static/{filepath}")).unwrap();

        response.status = 200;
        response.body = content.into_bytes();
    } else {
        let content = fs::read_to_string("static/404.html".to_string()).unwrap();
        response.status = 404;
        response.body = content.into_bytes();
        response.reason = "NOT FOUND".to_string();
    }

    response
        .headers
        .insert("Content-Type".to_string(), "text/html".to_string());

    send(response, &stream);
}

fn main() {
    listen(8080)
}
