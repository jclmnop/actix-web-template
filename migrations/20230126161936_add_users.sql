-- Add migration script here
CREATE TABLE users(
    username TEXT PRIMARY KEY,
    password TEXT NOT NULL
);