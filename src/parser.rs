//use clap::error::ErrorKind;
use reqwest::{Client};
use chrono::{DateTime, Utc, prelude::*};
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::error::Error;

use crate::configmanager::Config;

//API answer struct`s 
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: i32,
    pub current: Current,
    pub minutely: Option<Vec<Minutely>>,
    pub hourly: Option<Vec<Hourly>>,
    pub daily: Option<Vec<Daily>>,
    pub alerts: Option<Vec<Alert>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub dt: u64,
    pub sunrise: u64,
    pub sunset: u64,
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: u32,
    pub humidity: u32,
    pub dew_point: f32,
    pub uvi: f32,
    pub clouds: u32,
    pub visibility: u32,
    pub wind_speed: f32,
    pub wind_deg: u32,
    pub wind_gust: f32,
    pub weather: Vec<Weather>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Minutely {
    pub dt: u64,
    pub precipitation: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hourly {
    pub dt: u64,
    pub temp: f32,
    pub feels_like: f32,
    pub pressure: u32,
    pub humidity: u32,
    pub dew_point: f32,
    pub uvi: f32,
    pub clouds: u32,
    pub visibility: u32,
    pub wind_speed: f32,
    pub wind_deg: u32,
    pub wind_gust: f32,
    pub weather: Vec<Weather>,
    pub pop: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Daily {
    pub dt: u64,
    pub sunrise: u64,
    pub sunset: u64,
    pub moonrise: u64,
    pub moonset: u64,
    pub moon_phase: f32,
    pub summary: String,
    pub temp: Temp,
    pub feels_like: FeelsLike,
    pub pressure: u32,
    pub humidity: u32,
    pub dew_point: f32,
    pub wind_speed: f32,
    pub wind_deg: u32,
    pub wind_gust: f32,
    pub weather: Vec<Weather>,
    pub clouds: u32,
    pub pop: f32,
    pub rain: Option<f32>,
    pub uvi: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temp {
    pub day: f32,
    pub min: f32,
    pub max: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeelsLike {
    pub day: f32,
    pub night: f32,
    pub eve: f32,
    pub morn: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Alert {
    pub sender_name: String,
    pub event: String,
    pub start: u64,
    pub end: u64,
    pub description: String,
    pub tags: Vec<String>,
}

//type BoxedError = Box<dyn std::error::Error + Send + Sync>;

 
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