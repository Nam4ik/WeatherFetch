use std::{
    fs::{self, File},
    io::Read,
};

use reqwest::Client;
use toml;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_yml; 

// use crate::configmanager::Config;
use crate::shared::*;

fn get_config_path() -> Result<String, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    Ok(format!("{}/.config/WeatherFetch/Config.toml", home))
}

pub async fn parse_weather() -> Result<WeatherData, Box<dyn std::error::Error>> {
    let config = get_config()?;
    let client = Client::new();
    let response = client
        .get("https://api.open-meteo.com/v1/forecast")
        .query(&[
            ("latitude", config.lat.to_string()),
            ("longitude", config.lon.to_string()),
            ("current", "temperature_2m,wind_speed_10m".to_string()),
            (
                "hourly",
                "temperature_2m,relative_humidity_2m,wind_speed_10m".to_string(),
            ),
            ("timezone", config.timezone.unwrap_or("Europe/London".to_string())),
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

// TODO: Add exclude processing
#[derive(Debug, Deserialize)]
pub struct Config {
    lat: f64,
    lon: f64,
    exclude: String,
    timezone: Option<String>,
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

/*
pub fn generate_cachedir() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let cache_path = format!("{}/.cache/WeatherFetch/", home);

    let _ = fs::create_dir(cache_path);

    Ok(())
}
*/

/// Just writing example config to ~/.config/WeatherFetch/Config.toml
///     let _ = generate_config()
/// Result: ~/.config/WeatherFetch/Config.toml:
///     lat = 55.75
///     lon = 37.62
///     exclude = ""
pub fn generate_config() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path()?;
    let config = "lat = 55.75\nlon = 37.62\nexclude = \"\"\ntimezone = \"Europe/Moscow\"";
    let path = std::path::Path::new(&config_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, config)?;
    println!("Config file generated at: {}", config_path);
    Ok(())
}

/// For usage like type in `let`
#[derive(Serialize, Deserialize, Debug)]
struct Arts {
    #[serde(rename = "Arts")]
    arts: ArtsData,
}

/// Strings with arts from arts.yaml
#[derive(Serialize, Deserialize, Debug)]
struct ArtsData {
    sun: String,
    snow: String,
    rain: String,
}

/// Weather type checker, crutch, it hurts me to look at it
pub fn determine_weather_type(temp: f32, humidity: Option<u32>) -> &'static str {
    if temp < 0.0 {
        return "snow";
    }

    if let Some(hum) = humidity {
        if hum > 70 && temp < 25.0 {
            return "rain";
        }
    }

    "sun"
}

/// Arts loader with exception wrappers
fn load_arts(debug: bool) -> Result<ArtsData, Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let arts_path = format!("{}/.config/WeatherFetch/arts.yaml", home);
    
    let content = match fs::read_to_string(&arts_path) {
        Ok(c) => c,
        Err(_) => {
            // if arts.yaml not found
            return Ok(ArtsData {
                sun: "☀️ - ts emoji means program cant found ~/.config/WeatherFetch/arts.yaml"
                    .to_string(),
                snow: "❄️ - ts emoji means program cant found ~/.config/WeatherFetch/arts.yaml"
                    .to_string(),
                rain: "🌧️ - ts emoji means program cant found ~/.config/WeatherFetch/arts.yaml"
                    .to_string(),
            });
        }
    };

    if debug == true {
        println!("--- RAW FILE CONTENT START ---\n{}\n--- RAW FILE CONTENT END ---", content);
    } 

    let arts: Arts = serde_yml::from_str(&content)?;
    Ok(arts.arts)
}

fn process_placeholders(art: &str) -> String {
    let strart = art.to_string(); 
    let processed_art = strart.replace("{0}", " ") 
        .replace("<Yellow>", "\x1b[0;33m")
        .replace("<Blue>", "\x1b[0;34m")
        .replace("<Purple>", "\x1b[0;35m")
        .replace("<end>", "\x1b[0m");
    processed_art
}

/// Choosing and retuns art (String)
/// Usage:
///     let data: WeatherData = parse_cached()?;
///     prepare_art(&data);
pub fn prepare_art(weather_data: &WeatherData, debug: bool) -> Result<String, Box<dyn std::error::Error>> {
    let arts = load_arts(debug)?;

    let weather_type = determine_weather_type(
        weather_data.current.temperature_2m,
        weather_data.hourly.relative_humidity_2m.first().copied(),
    );

    let selected_art = match weather_type {
        "snow" => &arts.snow,
        "rain" => &arts.rain,
        "sun" => &arts.sun,
        _ => &arts.sun,
    };
    
    if debug == true {
        println!("-- RAW SELECTED ART (debug):\n{:?}\n", selected_art);
        println!("-- RAW SELECTED ART (display):\n{}\n", selected_art);
    }

    let processed_art = process_placeholders(selected_art);

    if debug == true { 
        println!("-- PROCESSED ART (debug):\n{:?}\n", processed_art);
        println!("-- PROCESSED ART (display):");
        println!("contains <Yellow>? {}", selected_art.contains("<Yellow>"));
        println!("contains <Purple>? {}", selected_art.contains("<Purple>"));
        println!("contains <Blue>? {}", selected_art.contains("<Blue>"));
        println!("processed contains \\x1b? {}", processed_art.contains("\x1b"));
    }
    Ok(processed_art)
}

/// Remove ANSI escape codes from string and return visible length
pub fn visible_length(s: &str) -> usize {
    let mut count = 0;
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' && chars.peek() == Some(&'[') {
            // Found ANSI escape sequence, skip until 'm'
            chars.next(); // consume '['
            while let Some(&next_ch) = chars.peek() {
                chars.next(); // consume character
                if next_ch == 'm' {
                    break; // end of ANSI sequence
                }
            }
        } else {
            count += 1;
        }
    }

    count
}


/// Left-pad a string with spaces to the specified visible width, accounting for ANSI codes
pub fn pad_with_ansi(s: &str, width: usize) -> String {
    let visible_len = visible_length(s);
    let padding_needed = if width > visible_len { width - visible_len } else { 0 };
    let padding = " ".repeat(padding_needed);
    format!("{}{}", s, padding)
}                               