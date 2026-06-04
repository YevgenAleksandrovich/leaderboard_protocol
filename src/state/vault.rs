use anchor_lang::prelude::*;

#[account]
pub struct RewardVault {
    pub reward_mint: Pubkey,

    pub total_deposited: u64,

    pub total_claimed: u64,

    pub bump: u8,
}