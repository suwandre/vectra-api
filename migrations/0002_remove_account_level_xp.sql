-- Add migration script here
ALTER TABLE users
DROP COLUMN account_xp,
DROP COLUMN account_level;