//use clap::error::ErrorKind;
use reqwest::{Client};
use chrono::{DateTime, Utc, prelude::*};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::{error::Error, fs::{read, File}, io::Read};
use toml::Deserializer; 

// use crate::configmanager::Config;
mod shared; 
use crate::shared::*; 

//fftype BoxedError = Box<dyn std::error::Error + Send + Sync>;

const DEFAULT_PATH: &str = "/home/$USER/.config/WeatherFetch/Config.toml";

const CONF: Lazy<Result<Config, Box<dyn std::error::Error + Send + Sync>>> = Lazy::new(|| {
    Config::load().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
});


pub fn get_location(coords_args: bool) -> Result<(), BoxedError> {
    let config = CONF
    .as_ref()
    .map_err(|e| Box::<dyn std::error::Error + Send + Sync>::from(e.to_string()))?
    .clone();

    if (config.lat == 0.0 || config.lon == 0.0) && !coords_args {
        
        println!("No coordinates in configuration file or conf not founded.");
        println!("HINT: Try create ~/.config/WeatherFetch/Config.toml");
        println!("HINT: And add `lat(<float>)`, `lon(<float>)`.");
        println!("HINT: To get more info check https://openweathermap.org/api/one-call-3");
        
        Err("Invalid coordinates in config".into())
    } else {
        Ok(())
    }
}

pub async fn parse_weather() -> Result<WeatherData, Box<dyn std::error::Error>> {
    let config = CONF
    .as_ref()
    .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?
    .clone();

    let client = Client::new();
    let response = client.get("https://api.openweathermap.org/data/3.0/onecall")
        .query(&[
            ("lat", config.lat.to_string()),
            ("lon", config.lon.to_string()),
            ("exclude", config.exclude),
            ("appid", config.appid),
            ("units", config.units),
            ("lang", config.lang),
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

pub fn get_config() -> Result<Config, dyn std::error::Error> {
    let mut file = read(DefaultConfig)?;
    let mut content: String = file.read_to_string(&mut contents)?;

    let mut deserialized_cfg = toml::Deserializer(&content)?;
    let config: Config = Deserialize::deserialize(&mut deserialized_cfg)?; 

    Ok(config)
 }