CREATE TABLE IF NOT EXISTS timers
(
    idx BIGSERIAL NOT NULL
        CONSTRAINT index_pk
            PRIMARY KEY,
    user_id uuid NOT NULL
        CONSTRAINT timer_to_users
        REFERENCES users(id)
        ON DELETE CASCADE,
    task VARCHAR(128) NOT NULL,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    started_at BIGINT NOT NULL,
    finished_at BIGINT
);