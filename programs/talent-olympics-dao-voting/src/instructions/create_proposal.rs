use anchor_lang::prelude::*;

use crate::{Proposal, HEADER_SIZE, PROPOSAL_IDENTIFIER};

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [PROPOSAL_IDENTIFIER, proposal_id.to_le_bytes().as_ref()],
        bump,
        space = HEADER_SIZE + Proposal::INIT_SPACE
    )]
    pub proposal: Account<'info, Proposal>,
    pub system_program: Program<'info, System>,
}
impl<'info> CreateProposal<'info> {
    pub fn execute(
        &mut self,
        proposal_id: u64,
        description: String,
        max_votes_allowed: u64,
        expiration_time: i64,
    ) -> Result<()> {
        self.proposal.set_inner(Proposal {
            id: proposal_id,
            max_votes_allowed,
            creator: self.user.to_account_info().key(),
            description,
            votes_in_favor: 0,
            votes_against: 0,
            expiration_time,
        });
        Ok(())
    }
}
