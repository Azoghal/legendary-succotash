-- Your SQL goes here
create table recipes (
    id serial primary key,
    title text not null,
    instructions text not null
);