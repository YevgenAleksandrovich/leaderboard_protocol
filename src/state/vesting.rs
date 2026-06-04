use anchor_lang::prelude::*;

#[account]
pub struct VestingSchedule {
    pub beneficiary: Pubkey,

    pub total_amount: u64,

    pub claimed_amount: u64,

    pub start_time: i64,

    pub cliff_time: i64,

    pub end_time: i64,

    pub bump: u8,
}