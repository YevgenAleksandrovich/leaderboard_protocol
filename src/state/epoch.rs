use anchor_lang::prelude::*;

#[account]
pub struct Epoch {
    pub id: u64,

    pub total_xp: u128,

    pub reward_pool: u64,

    pub reward_per_xp: u128,

    pub finalized: bool,

    pub start_time: i64,

    pub end_time: i64,

    pub bump: u8,
}