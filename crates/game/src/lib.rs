//! Gamification logic for Vectra DEX.
//! Handles XP calculations, level progression, and reward systems.
//! TODO: Update with actual calculations soon.

/// Calculates user level based on XP points.
/// Simple formula: level = (XP / 1000) + 1, capped at level 255.
pub fn calculate_level_from_xp(xp_points: u32) -> u8 {
    let level = (xp_points / 1000) + 1;
    std::cmp::min(level as u8, 255)
}

/// Calculates XP required for next level.
pub fn xp_required_for_next_level(current_level: u8) -> u32 {
    if current_level >= 255 {
        return 0; // Max level reached
    }
    (current_level as u32 + 1) * 1000
}

/// Calculates XP progress towards next level (0.0 to 1.0).
pub fn level_progress(xp_points: u32) -> f32 {
    let current_level = calculate_level_from_xp(xp_points);
    let current_level_xp = (current_level as u32 - 1) * 1000;
    let next_level_xp = current_level as u32 * 1000;
    
    if next_level_xp == current_level_xp {
        return 1.0; // Max level
    }
    
    (xp_points - current_level_xp) as f32 / (next_level_xp - current_level_xp) as f32
}
