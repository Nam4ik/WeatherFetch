use reqwest::Client;
use std::fs::File;
use std::io::Read;
use toml; 

// use crate::configmanager::Config;
use crate::shared::*; 

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

const DEFAULT_PATH: &str = "/home/$USER/.config/WeatherFetch/Config.toml";

/* 
const CONF: Lazy<Result<Config, Box<dyn std::error::Error + Send + Sync>>> = Lazy::new(|| {
    Config::load().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
});
*/

pub fn get_location(coords_args: bool) -> Result<(), BoxedError> {
    let config = get_config()?;

    if (config.lat == 0.0 || config.lon == 0.0) && !coords_args {
        
        println!("No coordinates in configuration file or conf not founded.");
        println!("HINT: Try create ~/.config/WeatherFetch/Config.toml");
        println!("HINT: And add `lat(<float>)`, `lon(<float>)`.");
        println!("HINT: To get more info check https://open-meteo.com/en/docs");
        
        Err("Invalid coordinates in config".into())
    } else {
        Ok(())
    }
}

pub async fn parse_weather() -> Result<WeatherData, Box<dyn std::error::Error>> {
    let config = get_config()?;
    let client = Client::new();
    let response = client.get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", config.lat.to_string()),
            ("longitude", config.lon.to_string()),
            ("current", "temperature_2m,wind_speed_10m"),
            ("hourly", "temperature_2m,relative_humidity_2m,wind_speed_10m"),
        ])
        .send()
        .await?;
    
    let weather_data: WeatherData = response.json().await?;
    Ok(weather_data)
}


pub struct Config {
    lat: f64,
    lon: f64,
    exclude: String,
    appid: String,
    units: String,
    lang: String,
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let mut file = File::open(DEFAULT_PATH)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content)?; 

    Ok(config)
}