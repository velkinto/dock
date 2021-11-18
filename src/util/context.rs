use std::net::TcpStream;
use crate::config::Config;
use super::receiver::Receiver;
use super::reactor::Reactor;

pub struct Context<'r> {
    config: &'r Config,
}

impl Context<'_> {
    pub fn new<'r>(config: &'r Config) -> Context {
        Context {
            config,
        }
    }

    pub fn process(&self, mut stream: TcpStream) {
        println!("new request!");
        let mut receiver = Receiver::new(&self.config, &mut stream);
        receiver.process();
        let trigger = receiver.trigger();
        let mut reactor = Reactor::new(&self.config, &mut stream, &trigger);
        reactor.process();
    }
}
