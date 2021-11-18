// mod trigger;
// mod http;
// mod util;
mod config;
mod listener;
mod util;
mod trigger;

use config::Config;
use listener::Listener;
use util::context::Context;

// fn main() {
//     let port: u32 = std::env::var("DOCK_PORT").unwrap_or("8000".to_owned()).parse().unwrap();
//     let listener = http::listen(port);

//     for stream in listener.incoming() {
//         http::receive(stream.unwrap());
//     }
// }

fn main() {
    let config: Config = Config::from("dock.toml").expect("配置文件不符合格式。");
    println!("{}", config.port);

    let listener = Listener::new(config.port);
    let context = Context::new(&config);

    for connection in listener.next() {
        context.process(connection.unwrap());
    }
}
