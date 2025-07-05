//! Gamification logic for Vectra DEX.
//! Handles XP calculations, level progression, and reward systems.

/// Calculates user level based on XP points.
/// Simple formula: level = (XP / 1000) + 1, capped at level 255.
pub fn calculate_level_from_xp(xp_points: u32) -> u8 {
    let level = (xp_points / 1000) + 1;
    std::cmp::min(level as u8, 255)
}

/// Helper function to safely convert i32 XP to u32 for calculations.
/// Handles negative values by treating them as 0.
pub fn safe_xp_conversion(xp_points: i32) -> u32 {
    if xp_points < 0 {
        0
    } else {
        xp_points as u32
    }
}

/// Calculates XP required for next level.
/// Takes current level (u8) and returns XP needed as u32.
pub fn xp_required_for_next_level(current_level: u8) -> u32 {
    if current_level >= 255 {
        return 0; // Max level reached
    }
    (current_level as u32 + 1) * 1000
}

/// Calculates XP progress towards next level (0.0 to 1.0).
/// Returns progress percentage as a float between 0.0 and 1.0.
pub fn level_progress(xp_points: u32) -> f32 {
    let current_level = calculate_level_from_xp(xp_points);
    
    // Handle max level case
    if current_level >= 255 {
        return 1.0;
    }
    
    let current_level_xp = (current_level.saturating_sub(1)) as u32 * 1000;
    let next_level_xp = current_level as u32 * 1000;
    
    // Calculate progress within current level
    let progress_in_level = xp_points.saturating_sub(current_level_xp);
    let xp_needed_for_level = next_level_xp - current_level_xp;
    
    if xp_needed_for_level == 0 {
        return 1.0; // Edge case protection
    }
    
    (progress_in_level as f32 / xp_needed_for_level as f32).min(1.0)
}

/// Calculates total XP required to reach a specific level.
/// Useful for displaying level requirements in UI.
pub fn xp_for_level(level: u8) -> u32 {
    if level <= 1 {
        return 0;
    }
    (level.saturating_sub(1)) as u32 * 1000
}
