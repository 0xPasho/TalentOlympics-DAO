use anchor_lang::prelude::*;

use crate::{error::CustomError, Proposal, PROPOSAL_IDENTIFIER};

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct TerminateProposal<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [PROPOSAL_IDENTIFIER, proposal_id.to_le_bytes().as_ref()],
        bump,
        constraint = proposal.creator == user.key(),
        close = user
    )]
    pub proposal: Account<'info, Proposal>,
}

impl<'info> TerminateProposal<'info> {
    pub fn execute(&mut self, _proposal_id: u64) -> Result<()> {
        require!(
            self.proposal.expiration_time.lt(&Clock::get()?.unix_timestamp),
            CustomError::ProposalStillActive
        );
        Ok(())
    }
}
