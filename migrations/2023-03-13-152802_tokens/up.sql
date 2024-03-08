-- Your SQL goes here
CREATE TABLE tokens
(
    token VARCHAR NOT NULL,
    user_id uuid NOT NULL
        CONSTRAINT tokens_to_users
        REFERENCES users
        ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL,
    last_used_at TIMESTAMP NOT NULL
);

CREATE UNIQUE INDEX tokens_token_uindex ON tokens (token);

ALTER TABLE tokens
    ADD CONSTRAINT tokens_pk
        PRIMARY KEY (token);