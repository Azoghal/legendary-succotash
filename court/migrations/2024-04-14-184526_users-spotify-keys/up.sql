-- Your SQL goes here
create table spotify_tokens (
    id serial primary key,
    user_id serial references users(id) on delete cascade,
    token text not null
);