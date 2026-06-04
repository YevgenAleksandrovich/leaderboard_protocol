use anchor_lang::prelude::*;

#[account]
pub struct Governance {
    pub authority: Pubkey,

    pub proposal_count: u64,

    pub voting_period: i64,

    pub quorum_bps: u16,

    pub bump: u8,
}