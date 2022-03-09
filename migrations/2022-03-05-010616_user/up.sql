-- Your SQL goes here
create table users(
    id uuid primary key not null,
    first_name varchar not null,
    last_name varchar not null,
    email varchar not null,
    username varchar not null,
    password varchar not null
)