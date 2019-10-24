use crate::schema::entries;
use chrono::{NaiveDate, NaiveTime};

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub day: NaiveDate,
    pub time_: NaiveTime, 
    pub machine: String,
    pub process: String,
    pub message: String
}

#[derive(Insertable)]
#[table_name="entries"]
pub struct NewEntry<'a> {
    pub day: &'a NaiveDate,
    pub time_: &'a NaiveTime, 
    pub machine: &'a str,
    pub process: &'a str,
    pub message: &'a str
}

