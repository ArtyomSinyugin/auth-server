-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
id uuid DEFAULT uuid_generate_v4() NOT NULL
    CONSTRAINT table_name_pk 
        PRIMARY KEY, 
username VARCHAR(128) NOT NULL,
access_rights INTEGER DEFAULT 1 NOT NULL,
company VARCHAR(255),
secret TEXT NOT NULL
);

CREATE UNIQUE INDEX users_id_uindex ON users (id);

CREATE UNIQUE INDEX users_username_uindex ON users (username);

