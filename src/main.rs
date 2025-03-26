extern crate image;


use clap::{Arg, Command}; 
use termimage::{Options};

mod configmanager;
mod parser;

use crate::configmanager::{Config, handle_config};
fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = Config::load()?;
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
            println!("Usage:");
            println!("VALUES");
            println!("--img=,  -i=  image, takees a path to .jpg/.png");
            println!("--lat=,  -t=, coordinates: takes f64.");
            println!("--lon=,  -n=, coordinates: takes f64");
            println!("--cfg=,  -c=, takes a path to Config.toml");
            println!("--exclude=, -e=, takes exlude type. See the API docs");
            println!("FUNCTIONS");
            println!("--gen-conf, -g, generating standart config");
            println!("--alerts, -a, show alerts");
    
            return Ok(());
        }
    
        if let Some(img_path) = matches.get_one::<String>("image") {
            let _img = image::open(img_path)?;
        }


    handle_config(&config)?;


    let _opts = Options::parse(); // Prefix with underscore if not used
  /*  
    if !opts {
        opts = configmanager::Config::load();
    } 

    let format = ops::guess_format(&opts.image)?;
    let img = ops::load_image(&opts.image, format)?;

  */

    Ok(())
}
