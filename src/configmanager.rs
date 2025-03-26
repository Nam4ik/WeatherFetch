use std::fs;
use serde::Deserialize;
use dirs::home_dir;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub lat:     f64,
    pub lon:     f64,
    pub exclude: String,
    pub appid:   String,
    pub lang:    String,
    pub units:   String,
    pub cache:   bool,
    pub rain:    String,
    pub sunny:   String,
    pub snowy:   String,
}

type BoxedError = Box<dyn std::error::Error + Send + Sync>;



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
        
        let config_str = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read config: {}", e))?;
        
        let config: Config = toml::from_str(&config_str)
            .map_err(|e| format!("Invalid TOML: {}", e))?;
        
        Ok(config)
    }
}


