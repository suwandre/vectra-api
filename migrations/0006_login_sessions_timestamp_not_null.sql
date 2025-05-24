-- 1) Backfill old rows
UPDATE login_sessions
  SET created_at = now()
 WHERE created_at IS NULL;

-- 2) Prevent future NULLs
ALTER TABLE login_sessions
  ALTER COLUMN created_at SET NOT NULL;