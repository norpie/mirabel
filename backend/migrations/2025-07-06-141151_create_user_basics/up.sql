-- Your SQL goes here

CREATE TABLE users (
    id text NOT NULL PRIMARY KEY,
    username text NOT NULL,
    email text NOT NULL,
    password text NOT NULL,
    created_at timestamp NOT NULL,
    modified_at timestamp NOT NULL
);

CREATE TABLE avatars (
    id text NOT NULL PRIMARY KEY,
    user_id text NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE auth_options (
    id text NOT NULL PRIMARY KEY,
    user_id text NOT NULL,
    two_factor_encoded text,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
