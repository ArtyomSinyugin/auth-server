-- Your SQL goes here
CREATE TABLE jobs
(
    job VARCHAR(128) NOT NULL,
    user_id uuid NOT NULL
        CONSTRAINT job_to_users
        REFERENCES users
        ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL,
    last_used_at TIMESTAMP NOT NULL,
    PRIMARY KEY (job, user_id)
);