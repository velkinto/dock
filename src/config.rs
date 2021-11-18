use toml;
use serde::Deserialize;

/// 配置类，包含一些列的读写方法
#[derive(Deserialize)]
pub struct Config {
    pub port: u32
}

impl Config {

    /// 默认配置
    const DEFAULT_CONFIG: Config = Config {
        port: 8000
    };

    /// 在当前工作目录下读取配置文件，如果没有找到配置文件，则返回默认配置。
    ///
    /// * `filename` - 配置文件名，存放在工作目录下。
    ///
    pub fn from(filename: &str) -> std::io::Result<Config> {
        match std::fs::read_to_string(filename) {
            Ok(contents) => {
                let config: Config = toml::from_str(&contents)?;
                Ok(config)
            },
            Err(_) => Ok(Config::DEFAULT_CONFIG)
        }
    }
}

