use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub openai_api_key: String,
    pub openai_api_url: String,
}

const CONFIG_FILE_NAME: &str = ".aicommit.toml";

pub fn get_config() -> Option<Config> {
    let home_path = dirs::home_dir();

    match home_path {
        Some(path) => {
            let mut config_str = String::new();

            let config_file = File::open(path.join(CONFIG_FILE_NAME));

            match config_file {
                Ok(mut file) => match file.read_to_string(&mut config_str) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error reading config file: {}", e);
                        return None;
                    }
                },
                Err(_) => {
                    println!("Config file not found");
                    return None;
                }
            }

            let config: Result<Config, toml::de::Error> = toml::from_str(&config_str);

            match config {
                Ok(config) => Some(config),
                Err(e) => {
                    println!("Error parsing config file: {}", e);

                    None
                }
            }
        }
        None => None,
    }
}
