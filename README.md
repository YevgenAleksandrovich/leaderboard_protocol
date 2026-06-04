leaderboard_protocol/
    ├── src/
    │   ├── lib.rs
    │   │
    │   ├── state/
    │   │   ├── governance.rs
    │   │   ├── epoch.rs
    │   │   ├── contributor.rs
    │   │   ├── reputation.rs
    │   │   ├── vesting.rs
    │   │   └── vault.rs
    │   │
    │   ├── instructions/
    │   │   ├── governance/
    │   │   │   ├── create_proposal.rs
    │   │   │   ├── vote.rs
    │   │   │   └── execute.rs
    │   │   │
    │   │   ├── epoch/
    │   │   │   ├── initialize_epoch.rs
    │   │   │   ├── finalize_epoch.rs
    │   │   │   └── rollover_epoch.rs
    │   │   │
    │   │   ├── contributor/
    │   │   │   ├── register.rs
    │   │   │   ├── update_xp.rs
    │   │   │   └── verify.rs
    │   │   │
    │   │   ├── rewards/
    │   │   │   ├── claim.rs
    │   │   │   ├── merkle_claim.rs
    │   │   │   └── vest_rewards.rs
    │   │   │
    │   │   └── oracle/
    │   │       ├── submit_xp.rs
    │   │       └── oracle_vote.rs
    │   │
    │   ├── events.rs
    │   ├── errors.rs
    │   ├── constants.rs
    │   └── utils/
    │       ├── merkle.rs
    │       ├── reward_math.rs
    │       └── reputation.rs
    │
    └── tests/
state/governance.rs
use anchor_lang::prelude::*;

#[account]
pub struct Governance {
    pub authority: Pubkey,

    pub proposal_count: u64,

    pub voting_period: i64,

    pub quorum_bps: u16,

    pub bump: u8,
}
state/epoch.rs
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
state/contributor.rs
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
state/reputation.rs
use anchor_lang::prelude::*;

#[account]
pub struct ReputationEngine {
    pub bronze_threshold: u64,

    pub silver_threshold: u64,

    pub gold_threshold: u64,

    pub platinum_threshold: u64,

    pub bump: u8,
}
state/vesting.rs
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
state/vault.rs
use anchor_lang::prelude::*;

#[account]
pub struct RewardVault {
    pub reward_mint: Pubkey,

    pub total_deposited: u64,

    pub total_claimed: u64,

    pub bump: u8,
}
Oracle Network

Замість одного oracle:

#[account]
pub struct OracleNetwork {
    pub oracles: Vec<Pubkey>,

    pub minimum_votes: u8,

    pub bump: u8,
}
Proposal
#[account]
pub struct Proposal {
    pub id: u64,

    pub proposer: Pubkey,

    pub title: String,

    pub description: String,

    pub yes_votes: u64,

    pub no_votes: u64,

    pub executed: bool,

    pub expires_at: i64,

    pub bump: u8,
}
Merkle Distribution
#[account]
pub struct MerkleEpoch {
    pub epoch_id: u64,

    pub merkle_root: [u8; 32],

    pub reward_pool: u64,

    pub claimed_bitmap: Vec<u8>,

    pub bump: u8,
}
Claim Engine
#[event]
pub struct RewardClaimed {
    pub user: Pubkey,

    pub epoch_id: u64,

    pub reward_amount: u64,
}
Reward Math

Окремий модуль:

pub fn calculate_reward(
    user_xp: u128,
    total_xp: u128,
    reward_pool: u64,
) -> u64 {

    user_xp
        .checked_mul(reward_pool as u128)
        .unwrap()
        .checked_div(total_xp)
        .unwrap() as u64
}

Формула:

Reward=
TotalXP
UserXP
	​

×RewardPool

Що це покаже на співбесіді

Така архітектура демонструє:

PDA design
DAO governance
Reward distribution
Reputation systems
Oracle consensus
Vesting schedules
Merkle proofs
Multi-epoch accounting
Token vaults
Event-driven architecture
Production-grade project organization
