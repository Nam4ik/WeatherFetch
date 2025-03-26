use std::fs;
use serde::Deserialize;
use dirs::home_dir;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub lat: String,
    pub lon: String,
    pub exclude: String,
    pub appid: String,
    pub lang: String,
    pub units: String,
    pub cache: bool,
    pub rain: String,
    pub sunny: String,
    pub snowy: String,
}

pub fn handle_config(_config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn gen_standard_conf() {
    // TODO: Implement
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = home_dir().ok_or("Home directory not found")?;
        path.push(".config/WeatherFetch/Config.toml");
        
        let config_str = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}

