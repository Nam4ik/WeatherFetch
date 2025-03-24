//In developing 🏗️

use serde::{Serialize, Deserialize};
use std::{env, path::Path};
use clap::{Arg, Command};
use termimage::{Image};

mod parser;
mod configmanagerж

use parser::{parse_weather, Weather};
use configmanager::{Config, handle_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let matches = Command::new("WeatherFetch")
        .version("0.1")
        .author("Borisov Alexey <arcanetmodl@gmail.com>")
        .about("Weather fetch like fastfetch with image and ASCII art support. Just useless pet project.")
        .arg(
            Arg::new("image")
                .short("i")
                .long("image")
                .value_name("PATH")
                .help("Path to image file")
                .takes_value(true),
        )
        .arg(
            Arg::new("exclude")
                .short("e")
                .long("exclude")
                .value_name("TYPE")
                .possible_values(["current", "minutely", "hourly", "daily", "alerts"])
                .help("Exclude specific weather data type")
                .takes_value(true),
        )
        .arg(
            Arg::new("help")
                .short("h")
                .long("help")
                .help("Print help information")
                .takes_value(false),
        )
        .arg(
            Arg::new(lat)
                .short("lt")
                .long("lat")
                .value_name("TWO DIGITS")
                .help("Coordinates for weather API call. Latitude.")
                .takes_value(true)
        )   
        .arg(
            Arg::new("lon")
                .short("ln")
                .long("lon")
                .value_name("TWO DIGITS")
                .help("Coordinates for weather API call. Latitude.")
                .takes_value(true)
        )
        .get_matches();

    
    if matches.is_present("help") {
        
        return Ok(());
    }

    
    if let Some(img_path) = matches.value_of("image") {
        let img = Image::from_path(img_path)?;

    }


    if let Some(exclude_type) = matches.value_of("exclude") {


    }


    let config = Config::load()?;
    handle_config(&config)?;

    Ok(())
}
    
