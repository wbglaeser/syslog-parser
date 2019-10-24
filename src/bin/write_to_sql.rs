extern crate sys_parser;

use self::sys_parser::*;
use std::io::{stdin, Read};
extern crate diesel;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use regex::Regex;
#[macro_use]
extern crate lazy_static;

use chrono::{NaiveDate, NaiveTime};

lazy_static! {static ref RE: Regex = Regex::new(r"([A-Z][a-z]{2}\s\d{2})\s(\d{2}:\d{2}:\d{2})\s([A-z0-9]+)\s([A-z0-9  \[\]\(\).\-]+):\s([A-z0-9 \.:,;\-_?!#'+*´`'()\[\]]+\n)").unwrap();}

fn main() {
    let connection = establish_connection();

    // Syslog Path
    let file_path = "/var/log/system.log";

    // Read in File
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse some stuff
    let matches = RE.captures_iter(&contents);    

    for cap in matches {
        let dateyear = format!("2019 {}", &cap[1]); 

        let date = NaiveDate::parse_from_str(&dateyear, "%Y %b %d").expect("date did not work");
        let time = NaiveTime::parse_from_str(&cap[2], "%H:%M:%S").expect("time did not wort");

        let entry = create_entry(&connection, &date, &time, &cap[3], &cap[4], &cap[5]);
    }
        
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
