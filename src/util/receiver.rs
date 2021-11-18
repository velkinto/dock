use std::net::TcpStream;
use std::io::{BufReader, BufRead, Read};
use crate::config::Config;
use regex::Regex;
use crate::trigger::Trigger;

pub struct Receiver<'r> {
    config: &'r Config,
    stream: &'r TcpStream,
    header: String,
    body: String,
}

impl Receiver<'_> {
    pub fn new<'r>(config: &'r Config, stream: &'r TcpStream) -> Receiver<'r> {
        Receiver {
            config,
            stream,
            header: String::new(),
            body: String::new()
        }
    }

    pub fn process(&mut self) {
        let rnrn = Regex::new(r".*?\r\n\r\n$").unwrap();
        let content_length = Regex::new(r"Content-Length: (\d+)\r\n").unwrap();

        let mut reader = BufReader::new(self.stream);

        while {
            reader.read_line(&mut self.header).unwrap();
            !rnrn.is_match(&self.header)
        } { }
        let length = match content_length.captures(&self.header) {
            Some(res) => {
                res.get(1).map_or("0", |m| m.as_str()).parse::<usize>().unwrap()
            },
            None => 0
        };
        let mut body_buf = vec![0u8; length];
        reader.read_exact(&mut body_buf).unwrap();
        self.body.push_str(&String::from_utf8(body_buf).unwrap());
    }

    pub fn trigger(&self) -> Trigger {
        serde_json::from_str(&self.body).unwrap()
    }
}
