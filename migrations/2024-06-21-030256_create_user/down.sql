-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS "users";
DROP TYPE role_type;
DROP INDEX IF EXISTS users_email_idx;