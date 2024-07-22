-- This file should undo anything in `up.sql`
ALTER TABLE jobs DROP CONSTRAINT job_ct;
DROP INDEX jobs_uindex;
DROP TABLE jobs;