-- Your SQL goes here
ALTER TABLE tokens ALTER column created_at SET DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE tokens ALTER column last_used_at SET DEFAULT CURRENT_TIMESTAMP;