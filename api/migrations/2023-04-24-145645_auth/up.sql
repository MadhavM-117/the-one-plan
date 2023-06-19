-- Your SQL goes here
CREATE TABLE users (
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone_number TEXT,
    password TEXT NOT NULL
);


CREATE TABLE auth_sessions (
    id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id),
    created_at TIMESTAMP NOT NULL,
    expires_at TIMESTAMP NOT NULL
);
