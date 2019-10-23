use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

lazy_static! {static ref RE: Regex = Regex::new(r"([A-Z][a-z]{2}\s\d{2})\s(\d{2}:\d{2}:\d{2})\s([A-z0-9]+)\s([A-z0-9 \[\]\(\).\-]+):\s([A-z0-9 \.:,;\-_?!#'+*´`'()\[\]]+\n)").unwrap();}

fn main() -> Result<(), Error> { 

    // Syslog Path
    let file_path = "/var/log/system.log";
    
    // Read in File
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse some stuff
    let matches = RE.captures_iter(&contents);
    for cap in matches {
        println!("NEW LINE:");
        println!("Date: {:?}", &cap[1]);
        println!("Time: {:?}", &cap[2]);
        println!("Machine: {:?}", &cap[3]);
        println!("Process: {:?}", &cap[4]);
        println!("Message: {}\n", &cap[5]);
    }

    Ok(())
}
