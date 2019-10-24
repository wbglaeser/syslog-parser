#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use chrono::{NaiveTime, NaiveDate};
use failure::{Error, format_err};

pub mod schema;
pub mod models;

use self::models::{NewEntry, Entry};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_entry<'a>(conn: &PgConnection, date: &'a NaiveDate, time: &'a NaiveTime, machine: &'a str, process: &'a str, message: &'a str) -> Entry {
    use schema::entries;

    let new_entry = NewEntry {
        day: date,
        time_: time,
        machine: machine,
        process: process,
        message: message
    };

    diesel::insert_into(entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving new post")
}
