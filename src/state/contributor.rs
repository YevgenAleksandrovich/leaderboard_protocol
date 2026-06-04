use anchor_lang::prelude::*;

#[account]
pub struct Contributor {
    pub authority: Pubkey,

    pub github_username: String,

    pub lifetime_xp: u128,

    pub reputation_score: u64,

    pub current_rank: Rank,

    pub total_rewards_claimed: u64,

    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum Rank {
    Bronze,
    Silver,
    Gold,
    Platinum,
}