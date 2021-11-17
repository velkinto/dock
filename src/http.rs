use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpStream, TcpListener};

use crate::trigger;
use crate::util;


pub fn listen(port: u32) -> TcpListener {
    TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap()
}

pub fn read(stream: &TcpStream) -> trigger::Trigger {
    let mut reader = BufReader::new(stream);
    let length = util::read_header(&mut reader);
    let body = util::read_body(&mut reader, length);
    util::parse_body(std::str::from_utf8(&body).unwrap())
}

pub fn write(stream: &TcpStream) {
    let mut writer = BufWriter::new(stream);
    writer.write("HTTP/1.1 200 OK\r\n\r\nOK".as_bytes()).unwrap();
    writer.flush().unwrap();
}

pub fn receive(stream: TcpStream) {
    let trigger = read(&stream);
    write(&stream);
    util::handle_trigger(trigger);
}