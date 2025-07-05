-- Migration to update user table types for improved precision
-- Updates XP points to BIGINT and money fields to NUMERIC for i128 support

-- Update XP points from INTEGER to BIGINT
ALTER TABLE users ALTER COLUMN xp_points TYPE BIGINT;

-- Update portfolio value from BIGINT to NUMERIC for i128 precision
-- NUMERIC(39,0) can handle the full range of i128 (-2^127 to 2^127-1)
ALTER TABLE users ALTER COLUMN portfolio_value_cents TYPE NUMERIC(39,0);

-- Update cash balance from BIGINT to NUMERIC for i128 precision
ALTER TABLE users ALTER COLUMN cash_balance_cents TYPE NUMERIC(39,0);

-- Update constraints to match new types
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_xp_points_positive;
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_portfolio_value_cents_positive;
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_cash_balance_cents_positive;

-- Re-add constraints with updated names and types
ALTER TABLE users 
ADD CONSTRAINT check_xp_points_positive CHECK (xp_points >= 0),
ADD CONSTRAINT check_portfolio_value_cents_positive CHECK (portfolio_value_cents >= 0),
ADD CONSTRAINT check_cash_balance_cents_positive CHECK (cash_balance_cents >= 0);

-- Update level constraint to allow wider range for i16
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_level_range;
ALTER TABLE users ADD CONSTRAINT check_level_range CHECK (level >= 1 AND level <= 32767);

-- Add comments for clarity
COMMENT ON COLUMN users.xp_points IS 'XP points (maps to Rust i64, range: -2^63 to 2^63-1)';
COMMENT ON COLUMN users.level IS 'User level (maps to Rust i16, range: 1 to 32767)';
COMMENT ON COLUMN users.portfolio_value_cents IS 'Portfolio value in cents (maps to Rust i128)';
COMMENT ON COLUMN users.cash_balance_cents IS 'Cash balance in cents (maps to Rust i128)';
