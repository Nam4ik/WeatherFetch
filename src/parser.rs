use reqwest::Client;
use std::fs::{self, File};
use std::io::Read;
use toml; 
use serde::Deserialize;
use serde_json;

// use crate::configmanager::Config;
use crate::shared::*; 

pub type BoxedError = Box<dyn std::error::Error + Send + Sync>;

fn get_config_path() -> Result<String, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    Ok(format!("{}/.config/WeatherFetch/Config.toml", home))
}

/* 
const CONF: Lazy<Result<Config, Box<dyn std::error::Error + Send + Sync>>> = Lazy::new(|| {
    Config::load().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
});
*/

pub fn get_location(coords_args: bool) -> Result<(), BoxedError> {
    let config = get_config().unwrap();

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
            ("current", "temperature_2m,wind_speed_10m".to_string()),
            ("hourly", "temperature_2m,relative_humidity_2m,wind_speed_10m".to_string()),
        ])
        .send()
        .await?;
    
    let response_text = response.text().await?;

    let weather_data: WeatherData = serde_json::from_str(&response_text)?;
    
    let home = std::env::var("HOME")?;
    let cache_dir = format!("{}/.cache/WeatherFetch", home);
    fs::create_dir_all(&cache_dir)?;
    
    let cache_path = format!("{}/weather.json", cache_dir);
    let json_data = serde_json::to_string_pretty(&weather_data)?;
    fs::write(&cache_path, json_data)?;
    
    Ok(weather_data)
}


#[derive(Debug, Deserialize)]
pub struct Config {
    lat: f64,
    lon: f64,
    exclude: String,
    appid: String,
    units: String,
    lang: String,
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    
    if File::open(&config_path).is_err() {
        generate_config()?; 
    }
     
    let mut file = File::open(&config_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let config: Config = toml::from_str(&content)?; 

    Ok(config)
}

pub fn generate_cachedir() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let cache_path = format!("{}/.cache/WeatherFetch/", home);
    
    let _ = fs::create_dir(cache_path); 

    Ok(())
}

pub fn generate_config() -> Result<(), Box<dyn std::error::Error>> { 
    let config_path = get_config_path()?;
    let config = "lat = 55.75\nlon = 37.62\nexclude = \"\"\nappid = \"\"\nunits = \"metric\"\nlang = \"ru\""; 
    let path = std::path::Path::new(&config_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, config)?;
    println!("Config file generated at: {}", config_path);
    Ok(())
}