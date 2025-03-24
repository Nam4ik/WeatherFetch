mod condfigmanager

use reqwest::{Error, Client, get};
use chrono::{DateTime, Utc, prelude::*};
use serde::{Serialize, Deserealize};
use std::net::{IpAddr, SocketAddr, UpdSocket};
use tokio;

static ErrBuff: String = reqwest::Error;

use serde::{Deserialize, Serialize};

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
:qa


:q

async fn get_location(let ip: String) -> Reslut<(), >{   
  let ip: String = reqwest::get("https://api.ipify.org").await?.text.await?;
} 


#[tokio::main]
async pub fn parse_wheather() -> Result<(), Error>{

}
