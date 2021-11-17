mod trigger;
mod http;
mod util;

fn main() {
    let listener = http::listen(8002);

    for stream in listener.incoming() {
        http::receive(stream.unwrap());
    }
}
