/*
  _______                                ________              __________
  ___    |__________________ _______________  __ \_______   __ ___  ___  \
  __  /| |_  ___/  ___/  __ `/_  __ \  _ \_  / / /  _ \_ | / / __  / _ \  |
  _  ___ |  /   / /__ / /_/ /_  / / /  __/  /_/ //  __/_ |/ /  _  / , _/ /
  /_/  |_/_/    \___/ \__,_/ /_/ /_/\___//_____/ \___/_____/   |_/_/|_|_/

MIT License

Copyright (c) 2023-2025 ArcaneDev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

*/

use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand}; 
use termimage::Options;

mod parser;
mod shared;

use parser::{get_config, parse_weather, Config}; 
use shared::WeatherData; 

#[derive(Parser)]
#[command(name = "wfetch")]
#[command(about = "Weather fetch tool")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Config, 
    Fetch,
    Clean, 
    Help,
    Today, 
    Tomorrow, 
    RebuildCache,
}

fn process_config() -> Result<(), Box<dyn std::error::Error>> {
    let _cfg: Config = get_config()?; 
    println!("Config is valid");
    Ok(()) 
}

fn parse_cached() -> Result<WeatherData, Box<dyn std::error::Error>> { 
    let home = std::env::var("HOME")?;
    let cache_path = format!("{}/.cache/WeatherFetch/weather.json", home);
    
    if !PathBuf::from(&cache_path).exists() {
        return Err("Cache file not found".into());
    }
    
    let cache_data = fs::read_to_string(&cache_path)?;
    let weather_data: WeatherData = serde_json::from_str(&cache_data)?;
    Ok(weather_data)
}

fn print_help() -> Result<(), Box<dyn std::error::Error>> {
    println!("Help command");
    println!("Usage: wfetch <command>");
    println!("Commands:");
    println!("  config      - Check config");
    println!("  fetch       - Fetch weather-data");
    println!("  clean       - Clean cache");
    println!("  help        - Print help");
    println!("  today       - Print today weather");
    println!("  tomorrow    - Print tomorrow weather");
    println!("  rebuild-cache - Rebuild cache");
    Ok(())
}

fn clean_cache() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let cache_path = format!("{}/.cache/WeatherFetch/weather.json", home);
    if PathBuf::from(&cache_path).exists() {
        fs::remove_file(&cache_path)?;
        println!("Cache cleaned successfully");
    } else {
        println!("Cache file not found");
    }
    Ok(())
}

fn rebuild_cache() -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new()?;
    let weather_data = rt.block_on(parse_weather())?;
    
    let home = std::env::var("HOME")?;
    let cache_dir = format!("{}/.cache/WeatherFetch", home);
    fs::create_dir_all(&cache_dir)?;
    
    let cache_path = format!("{}/weather.json", cache_dir);
    let json_data = serde_json::to_string_pretty(&weather_data)?;
    fs::write(&cache_path, json_data)?;
    
    println!("Cache rebuilt successfully");
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Config) => {
            println!("Config checker command");
            process_config()?;
            Ok(())
        },
        Some(Commands::Fetch) => {
            println!("Fetch weather-data command");
            let rt = tokio::runtime::Runtime::new()?;
            let weather_data = rt.block_on(parse_weather())?;
            println!("Weather data fetched: {:?}", weather_data);
            Ok(())
        },
        Some(Commands::Clean) => {
            println!("Clean cache command");
            clean_cache()?;
            Ok(())
        },
        Some(Commands::Help) => {
            print_help()
        },
        Some(Commands::Today) => {
            println!("Today weather command");
            Ok(())
        },
        Some(Commands::Tomorrow) => {
            println!("Tomorrow weather command");
            Ok(())
        },
        Some(Commands::RebuildCache) => {
            println!("Rebuild cache command");
            rebuild_cache()?;
            Ok(())
        },
        None => {
            print_help()
        },
    }
} 