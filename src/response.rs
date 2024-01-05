use std::collections::HashMap;

use itertools::Itertools;

pub struct Response {
    pub status: u8,
    pub protocol_version: String,
    pub reason: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: 200,
            protocol_version: String::from("HTTP/1.1"),
            reason: String::from("OK"),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
}

impl Response {
    pub fn as_bytes(&self) -> Vec<u8> {
        let Response {
            status,
            protocol_version,
            reason,
            headers,
            body,
        } = self;

        let formatted_headers = headers
            .iter()
            .map(|(k, v)| format!("{k}: {v}\r\n"))
            .join("\r\n");

        let mut formatted =
            format!("{protocol_version} {status} {reason}\r\n{formatted_headers}\r\n\r\n")
                .into_bytes();
        formatted.append(&mut body.clone());
        return formatted;
    }
}
