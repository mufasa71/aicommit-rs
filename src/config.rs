use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub gemini_api_key: String,
    pub gemini_api_url: String,
    pub model_name: String,
}

const CONFIG_FILE_NAME: &str = ".aicommit.toml";

pub fn get_config() -> Option<Config> {
    match dirs::home_dir() {
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

            let config: Config = toml::from_str(&config_str).expect("Failed to parse config file");

            Some(config)
        }
        None => None,
    }
}
