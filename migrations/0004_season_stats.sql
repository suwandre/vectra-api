-- 1. Create the new user_season_stats table
CREATE TABLE user_season_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    season INT NOT NULL DEFAULT 1,
    season_xp INT NOT NULL DEFAULT 0,
    season_level INT NOT NULL DEFAULT 1,
    updated_at TIMESTAMP DEFAULT now(),
    UNIQUE (user_id, season)
);

-- 2. Migrate existing XP and level values from users into season_stats
INSERT INTO user_season_stats (user_id, season, season_xp, season_level)
SELECT id, 1, season_xp, season_level FROM users;

-- 3. Drop the season_xp and season_level columns from users
ALTER TABLE users
DROP COLUMN season_xp,
DROP COLUMN season_level;