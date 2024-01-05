use std::collections::HashMap;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

use itertools::Itertools;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub protocol_version: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub(crate) fn from_stream(mut stream: &TcpStream) -> Request {
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
}
