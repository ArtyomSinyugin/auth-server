-- This file should undo anything in `up.sql`
DROP CONSTRAINT tokens_pk;
DROP INDEX tokens_token_uindex;
DROP TABLE tokens;