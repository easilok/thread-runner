use serde_derive::Deserialize;
use std::{collections::HashMap, fs};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub base_dir: String,
    pub command: String,
    pub environment: Option<Vec<HashMap<String, String>>>,
}

impl Config {
    pub fn build(filename: &str) -> Option<Config> {
        if let Ok(contents) = fs::read_to_string(filename) {
            if let Ok(data) = toml::from_str::<Config>(&contents) {
                return Some(data);
            }
        }

        None
    }
}
