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

use std::os::linux::process;

use clap::{Cli, Command, Subcommand}; 
use termimage;

mod parser;
use parser::{get_config, Config}; 

// use crate::configmanager::{Config, handle_config};
#[derive(Subcommand)]
enum Commands {
  Config, 
  Fetch,
  Clean, 
  Help,
  Today, 
  Tomorrow, 
  RebuildCache 
}
fn process_config() -> Result<Config, Box<dyn std::error::Error>> {
    let cfg: Config = get_config().unwrap(); 

    Ok(cfg); 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = process_config()?;
    let cli: Cli = Cli::parse();

    match cli.command {
        Commands::Config => {
            println!("Testing config."); // Заглушкии 
            // let test_result: bool = handle_config(config); 
        }
        Commands::Fetch => {
            println!("Fetching weather data.");
            
        }
        Commands::Clean => {
            println!("Clean");
        }
        Commands::Help => {
            println!("Help");
        }
        Commands::Today => {
            println!("Today");
        }
        Commands::Tomorrow => {
            println!("Tomorrow");
        }
        Commands::RebuildCache => {
        }
    } 

  }


