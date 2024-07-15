use anchor_lang::prelude::*;

#[account]
#[derive(InitialSize)]
pub struct Proposal {
    pub id: u64,
    pub creator: Pubkey,
    #[max_len(200)]
    pub description: String,
    pub max_votes_allowed: u64,
    pub votes_in_favor: u64,
    pub votes_against: u64,
    pub expiration_time: i64,
}
