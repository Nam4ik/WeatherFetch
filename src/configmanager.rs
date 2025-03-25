use std::fs;
use serde::Deserialize;
use dirs::home_dir;
use std::path::PathBuf;

impl WeatherFetch{
#[derive(Deserialize)]
pub struct Config {
   lat:     String, //Latitude, decimal (-90; 90)
   lon:     String, //Longitude, decimal (-180; 180)
   exclude: String, //By using this parameter you can exclude some parts of the weather 
   appid:   String, //Your OpenWeatherMap API key
   lang:    String, //Output language
   units:   String, //Units of measurement
   cache:   bool,   //Cacheing next Weather to dont use internet in next call
   rain:    String, //Path to rain image (png/jpg) or ASCII art int .txt
   sunny:   String, //Path to sunny image (png/jpg) or ASCII art int .txt 
   snowy:   String,
}
       //More info: OpenWeatherMap.org/api/one-call3

impl Config{
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let mut path = home_dir().ok_or("Не удалось найти домашнюю директорию")?;
        path.push(".config/WeatherFetch/Config.toml");
        
        let config_str = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }
}

pub fn handle_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // Пока недоделано
    Ok(())
}

pub fn gen_standart_conf() {

}

}

