-- migrations/0008_convert_timestamps_to_timestamptz.sql

-- 1) Users.created_at
ALTER TABLE users
  ALTER COLUMN created_at TYPE TIMESTAMPTZ
    USING created_at AT TIME ZONE 'UTC';
ALTER TABLE users
  ALTER COLUMN created_at SET DEFAULT now();

-- 2) Positions.opened_at
ALTER TABLE positions
  ALTER COLUMN opened_at TYPE TIMESTAMPTZ
    USING opened_at AT TIME ZONE 'UTC';
ALTER TABLE positions
  ALTER COLUMN opened_at SET DEFAULT now();

-- 3) Positions.closed_at
ALTER TABLE positions
  ALTER COLUMN closed_at TYPE TIMESTAMPTZ
    USING closed_at AT TIME ZONE 'UTC';
-- (no default on closed_at)

-- 4) Trades.created_at
ALTER TABLE trades
  ALTER COLUMN created_at TYPE TIMESTAMPTZ
    USING created_at AT TIME ZONE 'UTC';
ALTER TABLE trades
  ALTER COLUMN created_at SET DEFAULT now();

-- 5) XP_Log.created_at
ALTER TABLE xp_log
  ALTER COLUMN created_at TYPE TIMESTAMPTZ
    USING created_at AT TIME ZONE 'UTC';
ALTER TABLE xp_log
  ALTER COLUMN created_at SET DEFAULT now();

-- 6) User_Season_Stats.updated_at
ALTER TABLE user_season_stats
  ALTER COLUMN updated_at TYPE TIMESTAMPTZ
    USING updated_at AT TIME ZONE 'UTC';
ALTER TABLE user_season_stats
  ALTER COLUMN updated_at SET DEFAULT now();

-- 7) Login_Sessions.created_at
ALTER TABLE login_sessions
  ALTER COLUMN created_at TYPE TIMESTAMPTZ
    USING created_at AT TIME ZONE 'UTC';
ALTER TABLE login_sessions
  ALTER COLUMN created_at SET DEFAULT now();
