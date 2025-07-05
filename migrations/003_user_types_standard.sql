-- Revert to standard PostgreSQL types for better Rust compatibility
-- Changes NUMERIC back to BIGINT and ensures proper type alignment

-- Update portfolio value from NUMERIC to BIGINT
ALTER TABLE users ALTER COLUMN portfolio_value_cents TYPE BIGINT;

-- Update cash balance from NUMERIC to BIGINT  
ALTER TABLE users ALTER COLUMN cash_balance_cents TYPE BIGINT;

-- Update XP points from BIGINT back to INTEGER (for i32)
ALTER TABLE users ALTER COLUMN xp_points TYPE INTEGER;

-- Update constraints to match new types
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_xp_points_positive;
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_portfolio_value_cents_positive;
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_cash_balance_cents_positive;
ALTER TABLE users DROP CONSTRAINT IF EXISTS check_level_range;

-- Re-add constraints with proper ranges
ALTER TABLE users 
ADD CONSTRAINT check_xp_points_positive CHECK (xp_points >= 0),
ADD CONSTRAINT check_portfolio_value_cents_positive CHECK (portfolio_value_cents >= 0),
ADD CONSTRAINT check_cash_balance_cents_positive CHECK (cash_balance_cents >= 0),
ADD CONSTRAINT check_level_range CHECK (level >= 1 AND level <= 32767);

-- Update comments for clarity
COMMENT ON COLUMN users.xp_points IS 'XP points (maps to Rust i32, range: 0 to 2,147,483,647)';
COMMENT ON COLUMN users.level IS 'User level (maps to Rust i16, range: 1 to 32,767)';
COMMENT ON COLUMN users.portfolio_value_cents IS 'Portfolio value in cents (maps to Rust i64)';
COMMENT ON COLUMN users.cash_balance_cents IS 'Cash balance in cents (maps to Rust i64)';
