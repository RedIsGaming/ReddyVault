-- Your SQL goes here
CREATE TYPE roles AS ENUM ('admin', 'user', 'default');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created TIMESTAMP NOT NULL,
    role roles NOT NULL
);
