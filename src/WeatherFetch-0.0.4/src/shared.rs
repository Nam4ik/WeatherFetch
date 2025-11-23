use serde::{Deserialize, Serialize};
// API answer struct`s

/// Main struct with fetched weather data
#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: Option<f64>,
    pub utc_offset_seconds: Option<i32>,
    pub timezone: Option<String>,
    pub timezone_abbreviation: Option<String>,
    pub elevation: Option<f64>,
    pub current_units: Option<CurrentUnits>,
    pub current: Current,
    pub hourly_units: Option<HourlyUnits>,
    pub hourly: Hourly,
}

/// WeatherData.current = ts struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub time: String,
    pub temperature_2m: f32,
    pub wind_speed_10m: f32,
}

/// WeatherData.current_units = ts struct
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUnits {
    pub time: Option<String>,
    pub temperature_2m: Option<String>,
    pub wind_speed_10m: Option<String>,
}

/// WeatherData.hourly = ts struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    pub relative_humidity_2m: Vec<u32>,
    pub wind_speed_10m: Vec<f32>,
}

/// WeatherData.hourly_units = ts struct
#[derive(Debug, Serialize, Deserialize)]
pub struct HourlyUnits {
    pub time: Option<String>,
    pub temperature_2m: Option<String>,
    pub relative_humidity_2m: Option<String>,
    pub wind_speed_10m: Option<String>,
}
