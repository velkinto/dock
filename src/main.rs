use std::io::prelude::*;
use std::process::Command;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use regex::Regex;
use serde::{Deserialize, Serialize};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8002").unwrap();

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();

        handle_connection(stream);
    }
}

fn read_http(stream: &TcpStream) -> Trigger {
    let mut reader = BufReader::new(stream);
    let length = read_header(&mut reader);
    let body = read_body(&mut reader, length);
    parse_body(std::str::from_utf8(&body).unwrap())
}

fn write_http(stream: &TcpStream) {
    let mut writer = BufWriter::new(stream);
    writer.write("HTTP/1.1 200 OK\r\n\r\nOK".as_bytes()).unwrap();
    writer.flush().unwrap();
}

fn handle_connection(stream: TcpStream) {
    let trigger = read_http(&stream);
    write_http(&stream);
    let shell = format!("{}.{}.sh", trigger.repository.namespace, trigger.repository.name);
    let args = [shell, trigger.push_data.tag, trigger.push_data.digest, trigger.push_data.pushed_at];
    let output = Command::new("bash")
        .args(args)
        .output()
        .unwrap();
    println!("{}", String::from_utf8(output.stdout).unwrap());
    output.status.success();
}

fn read_header(reader: &mut BufReader<&std::net::TcpStream>) -> usize {
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

fn read_body(reader: &mut BufReader<&std::net::TcpStream>, length: usize) -> Vec<u8> {
    let mut body_buf = vec![0u8; length];
    reader.read_exact(&mut body_buf).unwrap();
    body_buf
}

#[derive(Serialize, Deserialize)]
struct PushData {
    digest: String,
    pushed_at: String,
    tag: String
}

#[derive(Serialize, Deserialize)]
struct Repository {
    date_created: String,
    name: String,
    namespace: String,
    region: String,
    repo_authentication_type: String,
    repo_full_name: String,
    repo_origin_type: String,
    repo_type: String,
}

#[derive(Serialize, Deserialize)]
struct Trigger {
    push_data: PushData,
    repository: Repository
}

fn parse_body(body: &str) -> Trigger {
    serde_json::from_str(body).unwrap()
}
