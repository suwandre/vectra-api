use crate::constants::{
    DEFAULT_CASHBACK_PERCENTAGE, DEFAULT_MAKER_FEE_PERCENTAGE, DEFAULT_TAKER_FEE_PERCENTAGE,
};

/// Showcases different benefits, fee adjustments and rewards obtained on specific season levels.
pub struct SeasonIncentives {
    /// Current level.
    pub level: u32,
    /// Fees paid in % when a limit order is executed).
    pub maker_fee_percentage: f32,
    /// Fees paid in % when a market order is executed).
    pub taker_fee_percentage: f32,
    /// Cashback rebate given back to the user after paying either a maker or taker fee for the trade (given back in % of the fees paid).
    ///
    /// NOTE: If the user did not pay any fees, this will be voided.
    pub cashback_percentage: f32,
}

/// Fetches the season incentives for a given season level.
pub fn get_season_incentives(level: u32) -> SeasonIncentives {
    match level {
        1..=10 => {
            let progress = ((level - 1) as f32) / 9f32;
            let maker_fee_percentage =
                DEFAULT_MAKER_FEE_PERCENTAGE + progress * (0.022 - DEFAULT_MAKER_FEE_PERCENTAGE);
            let taker_fee_percentage =
                DEFAULT_TAKER_FEE_PERCENTAGE + progress * (0.0515 - DEFAULT_TAKER_FEE_PERCENTAGE);
            let cashback_percentage =
                DEFAULT_CASHBACK_PERCENTAGE + progress * (1.2 - DEFAULT_CASHBACK_PERCENTAGE);
            SeasonIncentives {
                level,
                maker_fee_percentage,
                taker_fee_percentage,
                cashback_percentage,
            }
        }
        11..=20 => {
            let progress = ((level - 11) as f32) / 9f32;
            let maker_fee_percentage = 0.022 + progress * (0.021 - 0.022);
            let taker_fee_percentage = 0.0515 + progress * (0.05 - 0.0515);
            let cashback_percentage = 1.2 + progress * (1.5 - 1.2);
            SeasonIncentives {
                level,
                maker_fee_percentage,
                taker_fee_percentage,
                cashback_percentage,
            }
        }
        21..=40 => {
            let progress = ((level - 21) as f32) / 19f32;
            let maker_fee_percentage = 0.021 + progress * (0.019 - 0.021);
            let taker_fee_percentage = 0.05 + progress * (0.0475 - 0.05);
            let cashback_percentage = 1.5 + progress * (2.5 - 1.5);
            SeasonIncentives {
                level,
                maker_fee_percentage,
                taker_fee_percentage,
                cashback_percentage,
            }
        }
        41..=60 => {
            let progress = ((level - 41) as f32) / 19f32;
            let maker_fee_percentage = 0.019 + progress * (0.0165 - 0.019);
            let taker_fee_percentage = 0.0475 + progress * (0.044 - 0.0475);
            let cashback_percentage = 2.5 + progress * (4.5 - 2.5);
            SeasonIncentives {
                level,
                maker_fee_percentage,
                taker_fee_percentage,
                cashback_percentage,
            }
        }
        61..=100 => {
            let progress = ((level - 61) as f32) / 39f32;
            let maker_fee_percentage = 0.0165 + progress * (0.0125 - 0.0165);
            let taker_fee_percentage = 0.044 + progress * (0.039 - 0.044);
            let cashback_percentage = 4.5 + progress * (8.0 - 4.5);
            SeasonIncentives {
                level,
                maker_fee_percentage,
                taker_fee_percentage,
                cashback_percentage,
            }
        }
        101..=150 => {
            let progress = ((level - 101) as f32) / 49f32;
            let maker_fee_percentage = 0.0125 + progress * (0.0065 - 0.0125);
            let taker_fee_percentage = 0.039 + progress * (0.032 - 0.039);
            let cashback_percentage = 8.0 + progress * (15.0 - 8.0);
            SeasonIncentives {
                level,
                maker_fee_percentage,
                taker_fee_percentage,
                cashback_percentage,
            }
        }
        151..=200 => {
            let progress = ((level - 151) as f32) / 49f32;
            let maker_fee_percentage = 0.0065 + progress * (0.0 - 0.0065);
            let taker_fee_percentage = 0.032 + progress * (0.022 - 0.032);
            let cashback_percentage = 15.0 + progress * (30.0 - 15.0);
            SeasonIncentives {
                level,
                maker_fee_percentage,
                taker_fee_percentage,
                cashback_percentage,
            }
        }
        201..=255 => {
            let progress = ((level - 201) as f32) / 54f32;
            let maker_fee_percentage = 0.0; // stays constant
            let taker_fee_percentage = 0.022 + progress * (0.005 - 0.022);
            let cashback_percentage = 30.0 + progress * (50.0 - 30.0);
            SeasonIncentives {
                level,
                maker_fee_percentage,
                taker_fee_percentage,
                cashback_percentage,
            }
        }
        _ => {
            // Default to level 1 reward if out of range
            SeasonIncentives {
                level,
                maker_fee_percentage: DEFAULT_MAKER_FEE_PERCENTAGE,
                taker_fee_percentage: DEFAULT_TAKER_FEE_PERCENTAGE,
                cashback_percentage: DEFAULT_CASHBACK_PERCENTAGE,
            }
        }
    }
}
