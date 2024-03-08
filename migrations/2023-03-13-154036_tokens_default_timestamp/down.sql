-- This file should undo anything in `up.sql`
ALTER TABLE tokens ALTER column created_at SET DEFAULT NULL;
ALTER TABLE tokens ALTER column last_used_at SET DEFAULT NULL;