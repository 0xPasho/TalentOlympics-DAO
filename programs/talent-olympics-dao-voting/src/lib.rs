pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("5KqPscmVdEYJ9HmQdymcBvpfi515debCGUmpgoH6sEn4");

#[program]
pub mod dao_voting {
    use super::*;

    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        proposal_id: u64,
        description: String,
        max_votes_allowed: u64,
        expiration_time: i64,
    ) -> Result<()> {
        ctx.accounts.execute(proposal_id, description, max_votes_allowed, expiration_time)
    }

    pub fn submit_vote(ctx: Context<SubmitVote>, proposal_id: u64, against: bool) -> Result<()> {
        ctx.accounts.execute(proposal_id, against)
    }

    pub fn terminate_proposal(ctx: Context<TerminateProposal>, proposal_id: u64) -> Result<()> {
        ctx.accounts.execute(proposal_id)
    }
}
