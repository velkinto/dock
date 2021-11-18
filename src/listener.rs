use std::net::TcpListener;

pub struct Listener {
    tcp: TcpListener,
}

impl Listener {
    pub fn new(port: u32) -> Listener {
        return Listener {
            tcp: TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap()
        }
    }

    pub fn next(&self) -> std::net::Incoming {
        self.tcp.incoming()
    }
}
