/// Defines the XP thresholds per level range.
pub struct XpBracket {
    start_level: u32,
    end_level: u32,
    xp_per_level: u32,
}

/// Season-level brackets.
pub const SEASON_BRACKETS: &[XpBracket] = &[
    XpBracket { start_level: 1, end_level: 10, xp_per_level: 500 },
    XpBracket { start_level: 11, end_level: 20, xp_per_level: 5_000 },
    XpBracket { start_level: 21, end_level: 40, xp_per_level: 40_000 },
    XpBracket { start_level: 41, end_level: 60, xp_per_level: 100_000 },
    XpBracket { start_level: 61, end_level: 100, xp_per_level: 200_000 },
    XpBracket { start_level: 101, end_level: 150, xp_per_level: 400_000 },
    XpBracket { start_level: 151, end_level: 200, xp_per_level: 700_000 },
    XpBracket { start_level: 201, end_level: 255, xp_per_level: 1_100_000 },
];

/// Calculates season level based on XP.
pub fn calculate_season_level(xp: u32) -> u32 {
    let mut current_level = 1;
    let mut remaining_xp = xp;

    for bracket in SEASON_BRACKETS {
        let levels_in_bracket = bracket.end_level - bracket.start_level + 1;
        let bracket_total_xp = levels_in_bracket * bracket.xp_per_level;

        if remaining_xp >= bracket_total_xp {
            current_level = bracket.end_level + 1;
            remaining_xp -= bracket_total_xp;
        } else {
            let levels_earned = remaining_xp / bracket.xp_per_level;
            current_level += levels_earned;
            break;
        }
            
    }

    current_level.min(255)
}