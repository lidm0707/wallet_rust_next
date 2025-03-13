-- Your SQL goes here
CREATE TABLE
    users (
        id SERIAL PRIMARY KEY,
        username VARCHAR(255) NOT NULL UNIQUE,
        password_hash VARCHAR(255) NOT NULL,
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL,
        amount double precision NOT NULL DEFAULT 0
    );

CREATE TABLE
    trans (
        id SERIAL PRIMARY KEY,
        sender INTEGER NOT NULL,
        receiver INTEGER NOT NULL,
        created_at TIMESTAMP NOT NULL,
        updated_at TIMESTAMP NOT NULL,
        amount double precision NOT NULL DEFAULT 0
        -- amount DECIMAL(18, 2) NOT NULL DEFAULT 0
    );