use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

use itertools::Itertools;

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

struct Request {
    method: String,
    protocol_version: String,
    path: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

fn parse_req(mut stream: &TcpStream) -> Request {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request: Vec<String> = (&mut buf_reader)
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request_line = http_request.remove(0);

    let (method, path, version) = request_line
        .split_whitespace()
        .take(3)
        .collect_tuple()
        .unwrap();

    let headers: HashMap<_, _> = http_request
        .iter()
        .map(|line| {
            let parts = line.split_once(":");
            return parts.map(|(s1, s2)| (s1.trim(), s2.trim()));
        })
        .map(|item| item.unwrap())
        .map(|(s1, s2)| (s1.to_owned(), s2.to_owned()))
        .collect();

    let content_length = headers
        .get("Content-Length")
        .unwrap_or(&String::from("0"))
        .parse::<u8>()
        .unwrap();

    let mut buffer: Vec<u8> = Vec::with_capacity(content_length as usize);
    for _ in 0..buffer.capacity() {
        buffer.push(0u8)
    }

    if content_length > 0 {
        buf_reader.read_exact(&mut buffer).unwrap();
    }

    return Request {
        method: method.to_owned(),
        path: path.to_owned(),
        protocol_version: version.to_owned(),
        headers,
        body: buffer.to_owned(),
    };
}

fn handle_conn(mut stream: TcpStream) {
    let request = parse_req(&stream);

    println!("{:#?}", from_utf8(&request.body));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    listen(8080)
}
