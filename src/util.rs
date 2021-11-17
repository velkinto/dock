use std::io::prelude::*;
use std::io::{BufReader};
use regex::Regex;
use std::process::Command;

use crate::trigger;


pub fn read_header(reader: &mut BufReader<&std::net::TcpStream>) -> usize {
    let mut header = String::new();
    let seperator = Regex::new(r".*?\r\n\r\n$").unwrap();
    loop {
        reader.read_line(&mut header).unwrap();
        if seperator.is_match(&header) {
            break;
        }
    }
    let re_length = Regex::new(r"Content-Length: (\d+)\r\n").unwrap();
    match re_length.captures(& header) {
        Some(res) => {
            res.get(1).map_or("0", |m| m.as_str()).parse::<usize>().unwrap()
        },
        None => 0
    }
}

pub fn read_body(reader: &mut BufReader<&std::net::TcpStream>, length: usize) -> Vec<u8> {
    let mut body_buf = vec![0u8; length];
    reader.read_exact(&mut body_buf).unwrap();
    body_buf
}

pub fn parse_body(body: &str) -> trigger::Trigger {
    serde_json::from_str(body).unwrap()
}

pub fn handle_trigger(trigger: trigger::Trigger) {
    let shell = format!("{}.{}.sh", trigger.repository.namespace, trigger.repository.name);
    let args = [shell, trigger.push_data.tag, trigger.push_data.digest, trigger.push_data.pushed_at];
    let output = Command::new("bash")
        .args(args)
        .output()
        .unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    output.status.success();
}