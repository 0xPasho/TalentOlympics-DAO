use anchor_lang::prelude::*;

use crate::{error::CustomError, Member, Proposal, HEADER_SIZE, MEMBER_IDENTIFIER, PROPOSAL_IDENTIFIER};

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct SubmitVote<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,
    #[account(
        mut,
        seeds = [PROPOSAL_IDENTIFIER, proposal_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub proposal: Account<'info, Proposal>,
    #[account(
        init_if_needed,
        payer = participant,
        seeds = [MEMBER_IDENTIFIER, participant.key().as_ref()],
        space = HEADER_SIZE + Member::INITIAL_SIZE,
        bump
    )]
    pub member: Account<'info, Member>,
    pub system_program: Program<'info, System>,
}

impl<'info> SubmitVote<'info> {
    pub fn execute(&mut self, _proposal_id: u64, against: bool) -> Result<()> {
        require!(
            self.proposal.expiration_time.gt(&Clock::get()?.unix_timestamp),
            CustomError::ProposalExpired
        );

        require!(
            (self.proposal.votes_in_favor + self.proposal.votes_against).lt(&self.proposal.max_votes_allowed),
            CustomError::VoteLimitReached
        );

        if against {
            self.proposal.votes_against = self
                .proposal
                .votes_against
                .checked_add(1)
                .ok_or(CustomError::ArithmeticOverflow)?;
        } else {
            self.proposal.votes_in_favor = self
                .proposal
                .votes_in_favor
                .checked_add(1)
                .ok_or(CustomError::ArithmeticOverflow)?;
        }

        self.member.increment_points(1)?;
        Ok(())
    }
}
