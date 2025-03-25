//In developing 🏗️

use serde::{Serialize, Deserialize};
use std::{env, path::Path};
use clap::{Arg, Command};
use termimage::{Image};

mod parser;
mod configmanager;

use parser::{parse_weather, Weather};
use configmanager::{Config, handle_config};

f// main.rs
use clap::{Arg, Command};
use termimage::Image;

mod configmanager;
mod parser;

use configmanager::{Config, handle_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("WeatherFetch")
        .version("0.1")
        .author("Borisov Alexey <arcanetmodl@gmail.com>")
        .about("Weather fetch like fastfetch with image and ASCII art support")
        .arg(Arg::new("image").short('i').long("image").value_name("PATH"))
        .arg(Arg::new("exclude")
            .short('e')
            .long("exclude")
            .value_name("TYPE")
            .possible_values(["current", "minutely", "hourly", "daily", "alerts"]))
        .arg(Arg::new("help").short('h').long("help"))
        .arg(Arg::new("lat").short('t').long("lat").value_name("LATITUDE"))
        .arg(Arg::new("lon").short('n').long("lon").value_name("LONGITUDE"))
        .get_matches();

    if matches.contains_id("help") {
        println!("Usage: ...");
        return Ok(());
    }

    if let Some(img_path) = matches.get_one::<String>("image") {
        let _img = Image::from_path(img_path)?;
    }

    let config = Config::load()?;
    handle_config(&config)?;

    Ok(())
}