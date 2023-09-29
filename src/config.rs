use std::{env, fs};

use serde::{Deserialize, Serialize};

use crate::log::{log_error, log_info};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub token: String,
    pub database_uri: String,
    pub log_level: u8
}

pub fn check_conf() -> Config {
    let path = resources_path();
    let data = fs::read_to_string(path).unwrap();
    let config_data: Result<Config, serde_yaml::Error> = serde_yaml::from_str(&data);

    let config = match config_data {
        Ok(config_d) => config_d,
        Err(e) => {
            log_error("Error parsing config file!");
            log_error(&e.to_string());
            std::process::exit(0);
        }
    };
    log_info("Config has been located and loaded successfully!");
    config
}

fn resources_path() -> String {
    let mut path = env::current_exe().unwrap();
    path = path.canonicalize().unwrap();

    while path.pop() {
        path.push("config.yaml");
        if path.is_file() {
            break;
        }
        path.pop();

        path.push("config.yaml");
        if path.is_file() {
            break;
        }
        path.pop();
    }
    if !path.is_file() {
        log_error("Config file not found!");
        log_error("Are you sure you made the file?");
        std::process::exit(0);
    }
    return path.to_str().unwrap().to_string();
}
