-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tasks
(
    task VARCHAR(128) NOT NULL,
    user_id uuid NOT NULL
        CONSTRAINT task_to_users
        REFERENCES users
        ON DELETE CASCADE,
    created_at TIMESTAMP NOT NULL,
    last_used_at TIMESTAMP NOT NULL,
    PRIMARY KEY (task, user_id)
);

ALTER TABLE tasks ALTER column created_at SET DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE tasks ALTER column last_used_at SET DEFAULT CURRENT_TIMESTAMP;