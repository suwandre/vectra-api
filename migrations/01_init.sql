-- Stores user-based data.
CREATE TABLE users (
    id UUID PRIMARY KEY,
    wallet_address TEXT UNIQUE NOT NULL,
    account_xp INT NOT NULL DEFAULT 0,            -- all-time XP
    account_level INT NOT NULL DEFAULT 1,         -- persistent level
    season_xp INT NOT NULL DEFAULT 0,             -- resets per season
    season_level INT NOT NULL DEFAULT 1,          -- resets per season
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
-- Simulates open/closed trades (in perps).
CREATE TABLE positions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    symbol TEXT NOT NULL,
    side TEXT NOT NULL CHECK (side IN ('long', 'short')),
    entry_price FLOAT NOT NULL,
    margin_usd FLOAT NOT NULL,     -- amount the user committed (NOT notional/total)
    leverage FLOAT NOT NULL,
    opened_at TIMESTAMP NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMP,
    exit_price FLOAT
);

-- Represents a closed position/trade that's been settled.
CREATE TABLE trades (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    position_id UUID NOT NULL REFERENCES positions(id),
    pnl FLOAT NOT NULL,
    fee FLOAT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW() -- should be the same as positions closed_at
);

-- XP system tracking for each user, with sources.
CREATE TABLE xp_log (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    xp_source TEXT NOT NULL, -- e.g. "trade", "quest"
    amount INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
