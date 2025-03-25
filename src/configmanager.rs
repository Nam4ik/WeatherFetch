use std::fs;
use serde::Deserialize;

impl WeatherFetch{
#[derive(Deserialize)]
pub struct Config {
   lat:     String, //Latitude, decimal (-90; 90)
   lon:     String, //Longitude, decimal (-180; 180)
   exclude: String, //By using this parameter you can exclude some parts of the weather 
   appid:   String, //Your OpenWeatherMap API key
   lang:    String, //Output language
   units:   String, //Units of measurement
   cache:   bool    //Cacheing next Weather to dont use internet in next call
   rain:    String, //Path to rain image (png/jpg) or ASCII art int .txt
   sunny:   String, //Path to sunny image (png/jpg) or ASCII art int .txt 
   snowy:   String,
}
       //More info: OpenWeatherMap.org/api/one-call3

pub fn handle_config() -> Result<(), Box<dyn std::error::Error>>{
    let Config_str = fs::read_to_string("~/.config/WeatherFetch/Config.toml");
    let Config_parse: Config = toml::from_str(&data)?.except(); 
}
}

pub fn gen_standart_conf() {

}
