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

use std::{
    fs::{self, File},
    path::PathBuf,
};

use clap::{Parser, Subcommand};

mod parser;
mod shared;

use parser::{generate_config, 
             get_config, 
             parse_weather, 
             Config, 
             visible_length, 
             pad_with_ansi};

use shared::WeatherData;

use crate::parser::{determine_weather_type, prepare_art};

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
    Today,
    Tomorrow,
    RebuildCache,
    CheckCfg,
    Credits, 
    DebugOutput
}

/// Micro config-validator, easily you can just `wfetch fetch` and see the error
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

fn generate_weather_table_content(data: &WeatherData) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push("╔═══════════════════════════════════════╗".to_string());
    lines.push("║           Today`s weather             ║".to_string());
    lines.push("╠═══════════════════════════════════════╣".to_string());
    lines.push(format!(
        "║ Time: {}    ║",
        format!("{:>28}", data.current.time)
    ));
    lines.push(format!(
        "║ Temp: {}°C        ║",
        format!("{:>22}", data.current.temperature_2m)
    ));
    lines.push(format!(
        "║ Wind speed: {} m/s   ║",
        format!("{:>19}", data.current.wind_speed_10m)
    ));
    lines.push(format!(
        "║ Type:                  {}           ║",
        determine_weather_type(
            data.current.temperature_2m,
            Some(data.hourly.relative_humidity_2m[0])
        )
    ));
    if let Some(elevation) = data.elevation {
        lines.push(format!("║ Height: {} m  ║", format!("{:>26}", elevation)));
    }
    if let Some(timezone) = &data.timezone {
        lines.push(format!(
            "║ Time: {}          ║",
            format!("{:>22}", timezone)
        ));
    }
    lines.push(format!(
        "║ Coords: {:.2}, {:.2},                 ║",
        data.latitude, data.longitude
    ));
    lines.push("╚═══════════════════════════════════════╝".to_string());

    if !data.hourly.time.is_empty() {
        lines.push("┌──────────────────┬──────────────┬──────────────┬──────────────┐".to_string());
        lines.push("│ Time             │ Temp         │ Humidity     │ Wind         │".to_string());
        lines.push("├──────────────────┼──────────────┼──────────────┼──────────────┤".to_string());

        let hours_to_show = data.hourly.time.len().min(24);
        for i in 0..hours_to_show {
            let time = &data.hourly.time[i];
            let temp = data.hourly.temperature_2m[i];
            let humidity = data.hourly.relative_humidity_2m[i];
            let wind = data.hourly.wind_speed_10m[i];

            lines.push(format!(
                "│ {:12} │ {:>10}°C │ {:>10}% │ {:>10} m/s │",
                time, temp, humidity, wind
            ));
        }
        lines.push("└──────────────────┴──────────────┴──────────────┴──────────────┘".to_string());
    }

    lines
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Config) => {
            println!("Config checker command");
            let home = std::env::var("HOME")?;
            let config_path = format!("{}/.config/WeatherFetch/Config.toml", home);
            if File::open(&config_path).is_err() {
                generate_config()?;
                println!("Config generated successfully");
            } else {
                process_config()?;
                println!("Config already exists and is valid.");
            }
            Ok(())
        }
        Some(Commands::Fetch) => {
            println!("Fetch weather-data command");
            let rt = tokio::runtime::Runtime::new()?;
            let _weather_data = rt.block_on(parse_weather())?;
            let home = std::env::var("HOME")?;
            println!(
                "Weather data fetched to: {}",
                format!("{}/.cache/WeatherFetch", home)
            );
            Ok(())
        }
        Some(Commands::Clean) => {
            println!("Clean cache command");
            clean_cache()?;
            Ok(())
        }
        Some(Commands::Today) => {
            // to much vars
            let data: WeatherData = parse_cached()?;
            let art_string = prepare_art(&data, false)?;
            let table_lines = generate_weather_table_content(&data);

            let art_lines: Vec<&str> = art_string.lines().collect();

            let max_art_width = art_lines.iter().map(|line| visible_length(line)).max().unwrap_or(0);

            let max_lines = art_lines.len().max(table_lines.len());

            for i in 0..max_lines {
                let art_part = art_lines.get(i).unwrap_or(&"");
                let table_part = table_lines.get(i).map(|s| s.as_str()).unwrap_or("");

                let padded_art = pad_with_ansi(art_part, max_art_width);
                println!("{} {}", padded_art, table_part);
            }
            
            Ok(())
        }
        Some(Commands::Tomorrow) => {
            println!("Tomorrow weather command");
            Ok(())
        }
        Some(Commands::DebugOutput) => {
            let data: WeatherData = parse_cached()?;
            let art_string = prepare_art(&data, true)?;
            let table_lines = generate_weather_table_content(&data);

            let art_lines: Vec<&str> = art_string.lines().collect();

            let max_art_width = art_lines.iter().map(|line| visible_length(line)).max().unwrap_or(0);

            let max_lines = art_lines.len().max(table_lines.len());

            for i in 0..max_lines {
                let art_part = art_lines.get(i).unwrap_or(&"");
                let table_part = table_lines.get(i).map(|s| s.as_str()).unwrap_or("");

                let padded_art = pad_with_ansi(art_part, max_art_width);
                println!("{} {}", padded_art, table_part);
            }
            Ok(())
        }
        Some(Commands::RebuildCache) => {
            println!("Rebuild cache command");
            rebuild_cache()?;
            Ok(())
        }
        Some(Commands::CheckCfg) => {
            println!("Validating cfg...");
            let home = std::env::var("HOME")?;
            let config_path = format!("{}/.config/WeatherFetch/Config.toml", home);
            if File::open(&config_path).is_ok() {
                process_config()?;
            } else {
                println!(
                    "Config file not found, try `wfetch config`, its will generate default cfg."
                );
            }
            Ok(())
        }
        Some(Commands::Credits) => {
            println!("Credits:"); 
            println!("Maintaner & developer: Namilsk <namilsk@namilsk.tech>");
            println!("ASCII arts by www.asciiart.eu: Joan G. Stark");
            Ok(())
        }
        None => {
            println!("No subcommand specified.");
            println!("Run `wfetch -h` or `wfetch help`");
            println!("to see help message.");
            Ok(())
        }
    }
}
