mod trigger;
mod http;
mod util;

fn main() {
    let port: u32 = std::env::var("DOCK_PORT").unwrap_or("8000".to_owned()).parse().unwrap();
    let listener = http::listen(port);

    for stream in listener.incoming() {
        http::receive(stream.unwrap());
    }
}
