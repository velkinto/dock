use std::net::TcpStream;
use std::process::Command;
use std::io::{BufWriter, Write};
use crate::config::Config;
use crate::trigger::Trigger;

pub struct Reactor<'r> {
    config: &'r Config,
    stream: &'r TcpStream,
    trigger: &'r Trigger,
}

impl Reactor<'_> {
    pub fn new<'r>(config: &'r Config, stream: &'r TcpStream, trigger: &'r Trigger) -> Reactor<'r> {
        Reactor {
            config,
            stream,
            trigger,
        }
    }

    pub fn process(&mut self) {
        let mut writer = BufWriter::new(self.stream);
        writer.write("HTTP/1.1 200 OK\r\n\r\nOK".as_bytes()).unwrap();
        writer.flush().unwrap();
        let shell = format!("{}.{}.sh", self.trigger.repository.namespace, self.trigger.repository.name);
        let args = [
            shell,
            self.trigger.push_data.tag.to_owned(),
            self.trigger.push_data.digest.to_owned(),
            self.trigger.push_data.pushed_at.to_owned()
        ];
        let output = Command::new("bash")
            .args(args)
            .output()
            .unwrap();
        println!("{}", String::from_utf8(output.stdout).unwrap());
        output.status.success();
    }
}
