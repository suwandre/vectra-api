-- Create an index on created_at so your TTL checks are fast.
CREATE INDEX IF NOT EXISTS idx_login_sessions_created_at
  ON login_sessions (created_at);