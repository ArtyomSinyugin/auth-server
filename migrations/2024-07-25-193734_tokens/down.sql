-- This file should undo anything in `up.sql`
ALTER TABLE tokens ALTER column created_at SET DEFAULT NULL;
ALTER TABLE tokens ALTER column last_used_at SET DEFAULT NULL;
DROP CONSTRAINT tokens_pk;
DROP INDEX tokens_token_uindex;
DROP TABLE tokens;