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
use log::{debug, info, trace};
use dotenv::dotenv;
use std::env;

lazy_static! {static ref RE: Regex = Regex::new(r"([A-Z][a-z]{2}\s\d{2})\s(\d{2}:\d{2}:\d{2})\s([A-z0-9]+)\s([A-z0-9  \[\]\(\).\-]+):\s([A-z0-9 \.:,;\-_=?!#'+*´`'()\[\]/]+\n)").unwrap();}

fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    info!("Collect Syslog - v0.0.1");

    trace!("Starting connection ...");
    let connection = establish_connection();
    debug!("PgConnection is establised");

    // Syslog Path
    let file_path = "/var/log/system.log";

    // Read in File
    trace!("Read in system.log file");
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    debug!("File has been read to new string"); 

    // Parse some stuff
    trace!("Regex over message");
    let matches = RE.captures_iter(&contents);    
    debug!("Captures have been extracted");

    trace!("Starting loop over lines");
    for cap in matches {

        let dateyear = format!("2019 {}", &cap[1]); 

        let date = NaiveDate::parse_from_str(&dateyear, "%Y %b %d").expect("date did not work");
        let time = NaiveTime::parse_from_str(&cap[2], "%H:%M:%S").expect("time did not wort");

        create_entry(&connection, &date, &time, &cap[3], &cap[4], &cap[5]);
    }
        
    debug!("All entries read to database");
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";
