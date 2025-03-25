use reqwest::{Error, Client, get};
use chrono::{DateTime, Utc, prelude::*};
use serde::{Serialize, Deserealize};
use std::net::{IpAddr, SocketAddr, UpdSocket};

mod configmanager;

use configmanager::*;

static ErrBuff: String = reqwest::Error;

//API answer struct`s 
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: i32,
    pub current: Current,
    pub minutely: Vec<Minutely>,
    pub hourly: Vec<Hourly>,
    pub daily: Vec<Daily>,
    pub alerts: Vec<Alert>,
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





pub fn get_location(let coords_args: bool, config: &Config) -> Reslut<(), >{   
    //Get the lat and lon for API call 
    configmanager::handle_config();
    if configmanager::Config.lat.is_empty() || configmanager::Config.lon.is_empty() && !coords_args{
        println!("No coordinates in configuration file or conf not founded.");
        println!("HINT: Try create ~/.config/WeatherFetch/Config.toml");
        println!("HINT: And add `lat(<int>)`, `lon(<int>)`.");
        println!("HINT: To get more info check https://openweathermap.org/api/one-call-3")
        
        Err("No coordinates in config or args.".into())
    } else {
        Ok(())
    }
} 



pub async fn parse_weather(config: &Config) -> Result<(), reqwest::Error> {
    
    Ok(())
}