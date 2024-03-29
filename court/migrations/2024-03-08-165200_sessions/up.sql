-- Your SQL goes here

-- We also want is a user id foreign key
-- this way we can find the sessions for a user
-- and find the user from a session's hash

create table sessions (
    id serial primary key,
    user_id serial not null references users(id),
    expires int not null, 
    jwt_hash text not null unique,
    jwt text not null
);
