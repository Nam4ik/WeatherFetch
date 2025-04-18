extern crate image;


use clap::{Arg, Command}; 
use termimage::{Options};


mod configmanager;
mod parser;

use crate::configmanager::{Config, handle_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("WeatherFetch")
        .version("0.1")
        .author("Borisov Alexey <arcanetmodl@gmail.com>")
        .about("Weather fetch with image and ASCII art support")
        .arg(Arg::new("image").short('i').long("image").value_name("PATH"))
        .arg(Arg::new("exclude").short('e').long("exclude").value_name("TYPE")
        .value_parser(["current", "minutely", "hourly", "daily", "alerts"]))
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


    let opts = Options::parse();
  /*  
    if !opts {
        opts = configmanager::Config::load();
    } 

    let format = ops::guess_format(&opts.image)?;
    let img = ops::load_image(&opts.image, format)?;

  */

    Ok(())
}
