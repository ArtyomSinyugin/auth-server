-- This file should undo anything in `up.sql`
ALTER TABLE jobs ALTER column created_at SET DEFAULT NULL;
ALTER TABLE jobs ALTER column last_used_at SET DEFAULT NULL;