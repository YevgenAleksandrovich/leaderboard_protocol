use anchor_lang::prelude::*;

#[account]
pub struct ReputationEngine {
    pub bronze_threshold: u64,

    pub silver_threshold: u64,

    pub gold_threshold: u64,

    pub platinum_threshold: u64,

    pub bump: u8,
}