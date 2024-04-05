-- Your SQL goes here
create table users (
    id serial primary key,
    auth0subject text unique not null,
    name text not null 
);