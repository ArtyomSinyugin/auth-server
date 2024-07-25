-- This file should undo anything in `up.sql`
ALTER TABLE tasks ALTER column created_at SET DEFAULT NULL;
ALTER TABLE tasks ALTER column last_used_at SET DEFAULT NULL;
ALTER TABLE tasks DROP CONSTRAINT task_to_users;
DROP TABLE tasks;