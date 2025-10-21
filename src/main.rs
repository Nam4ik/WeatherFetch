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

extern crate image;


use clap::{Arg, Command}; 
use termimage::{Options};

mod parser;
use parser::{get_config, Config}; 

// use crate::configmanager::{Config, handle_config};

fn process_config() -> Result<Config, Box<dyn std::error::Error>> {
    get_config(); 

    Ok(Config::load()?)
}

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
