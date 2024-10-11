use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;
use std::process::exit;

#[derive(Deserialize)]
pub struct Config {
    pub save_dir: String,
}

fn load_config() -> Config {
    let home = std::env::var("HOME").unwrap();
    let path = format!("{}/.config/meowlog/config.toml", &home);
    let contents = match fs::read_to_string(path.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not read config file `{}`, with error: {:?}", path, e);
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(conf) => conf,
        Err(err) => {
            eprintln!("Unable to load data from file `{}`", path);
            eprintln!("=> {}", err.message());
            exit(1);
        }
    };

    config
}

lazy_static! {
    pub static ref CONFIG: Config = load_config();
    pub static ref HOME: String = std::env::var("HOME").unwrap();
    pub static ref LOCAL_PATH: String = format!("{}/.local/share/meowlog", HOME.to_string());
    pub static ref SUBSTANCES_FILE: String =
        format!("{}/substances.bin", LOCAL_PATH.to_string()).to_string();
    pub static ref INGESTIONS_FILE: String =
        format!("{}/ingestions.bin", LOCAL_PATH.to_string()).to_string();
}
