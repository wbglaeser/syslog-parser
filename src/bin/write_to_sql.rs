extern crate sys_parser;

use self::sys_parser::*;
use std::io::{stdin, Read};

extern crate sys_parser;
extern crate diesel;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

lazy_static! {static ref RE: Regex = Regex::new(r"([A-Z][a-z]{2}\s\d{2})\s(\d{2}:\d{2}:\d{2})\s([A-z0-9]+)\s([A-z0-9  \[\]\(\).\-]+):\s([A-z0-9 \.:,;\-_?!#'+*´`'()\[\]]+\n)").unwrap();}

fn main() {
    let connection = establish_connection();

    // Syslog Path
    let file_path = "/var/log/system.log";

    // Read in File
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse some stuff
    let matches = RE.captures_iter(&contents);    

    for cap in matches {
        let entry = create_entry(&connection, &cap[1], &cap[2], &cap[3], &cap[4], &cap[5]);
    }
        
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
