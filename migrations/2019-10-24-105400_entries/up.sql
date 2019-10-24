-- Your SQL goes here
CREATE TABLE entries (
    id Serial PRIMARY KEY,
    day Date NOT NULL,
    time_ Time NOT NULL,
    machine TEXT NOT NULL,
    process TEXT NOT NULL,
    message TEXT NOT NULL
)

