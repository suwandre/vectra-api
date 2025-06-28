-- Initial database schema for Vectra DEX
-- Creates core tables for users, trades, positions, and sessions

-- Users table for wallet-based authentication
CREATE TABLE users (
    id UUID PRIMARY KEY,
    wallet_address VARCHAR(42) UNIQUE NOT NULL,
    username VARCHAR(50),
    xp_points INTEGER NOT NULL DEFAULT 0,              -- Changed to INTEGER for u32
    level SMALLINT NOT NULL DEFAULT 1,                 -- Changed to SMALLINT for u8
    portfolio_value_cents BIGINT NOT NULL DEFAULT 1000000,   -- ✅ Added _cents suffix
    cash_balance_cents BIGINT NOT NULL DEFAULT 1000000,      -- ✅ Added _cents suffix
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add constraints for Rust type safety
ALTER TABLE users 
ADD CONSTRAINT check_xp_points_positive CHECK (xp_points >= 0),
ADD CONSTRAINT check_level_range CHECK (level >= 1 AND level <= 255),
ADD CONSTRAINT check_portfolio_value_cents_positive CHECK (portfolio_value_cents >= 0),  -- ✅ Updated
ADD CONSTRAINT check_cash_balance_cents_positive CHECK (cash_balance_cents >= 0);        -- ✅ Updated

-- Index for fast wallet address lookups
CREATE INDEX idx_users_wallet_address ON users(wallet_address);

-- Trades table for paper trading history
CREATE TABLE trades (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    symbol VARCHAR(10) NOT NULL,
    trade_type VARCHAR(4) NOT NULL CHECK (trade_type IN ('buy', 'sell')),
    quantity BIGINT NOT NULL,           -- Store as micro units
    price BIGINT NOT NULL,              -- Price in cents
    total_value BIGINT NOT NULL,        -- Total in cents
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for user trade history queries
CREATE INDEX idx_trades_user_id ON trades(user_id);
CREATE INDEX idx_trades_executed_at ON trades(executed_at);

-- Positions table for current portfolio holdings
CREATE TABLE positions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    symbol VARCHAR(10) NOT NULL,
    quantity BIGINT NOT NULL,           -- Quantity in micro units
    average_price BIGINT NOT NULL,      -- Average price in cents
    current_value BIGINT NOT NULL,      -- Current value in cents
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, symbol)
);

-- Index for portfolio queries
CREATE INDEX idx_positions_user_id ON positions(user_id);

-- User sessions table for JWT token management
CREATE TABLE user_sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for session lookups
CREATE INDEX idx_user_sessions_token_hash ON user_sessions(token_hash);
CREATE INDEX idx_user_sessions_expires_at ON user_sessions(expires_at);
