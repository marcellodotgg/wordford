create table users (
    id integer primary key autoincrement, 
    given_name text not null,
    family_name text not null,
    email text not null unique,
    avatar_url text not null,
    password_hash text not null,
    role integer not null default 1,
    created_at datetime not null default current_timestamp,
    updated_at datetime not null default current_timestamp
);
