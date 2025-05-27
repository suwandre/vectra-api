CREATE TABLE refresh_tokens (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  user_id     UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token_hash  TEXT NOT NULL,                       -- store a hash, not the raw token
  expires_at  TIMESTAMPTZ NOT NULL,                -- when this refresh token can no longer be used
  created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Index for lookup by user_id + token_hash
CREATE INDEX idx_refresh_tokens_user_hash
  ON refresh_tokens (user_id, token_hash);