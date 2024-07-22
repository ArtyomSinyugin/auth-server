-- Your SQL goes here
CREATE TABLE timers
(
    idx VARCHAR NOT NULL
        CONSTRAINT index_pk
            PRIMARY KEY,
    user_id uuid NOT NULL
        CONSTRAINT timer_to_users
        REFERENCES users(id)
        ON DELETE CASCADE,
    job VARCHAR(128) NOT NULL,
    date DATE NOT NULL DEFAULT CURRENT_DATE,
    started_at VARCHAR(128) NOT NULL,
    finished_at VARCHAR(128)
);

CREATE UNIQUE INDEX timers_uindex ON timers (idx);