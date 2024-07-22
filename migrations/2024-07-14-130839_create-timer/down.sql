-- This file should undo anything in `up.sql`
ALTER TABLE timers DROP CONSTRAINT timer_to_users;
DROP INDEX timers_uindex;
DROP TABLE timers;