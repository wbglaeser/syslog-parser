use schema::entries;

#[derive(Queryable)]
pub struct Entry {
    pub id: i32,
    pub date: String,
    pub time: String, 
    pub machine: String,
    pub process: String,
    pub message: String
}

#[derive(Insertable)]
#[table_name="entries"]
pub struct NewEntry<'a> {
    pub date: &'a str,
    pub time: &'a str,
    pub machine: &'a str,
    pub process: &'a str,
    pub message: &'a str
}
