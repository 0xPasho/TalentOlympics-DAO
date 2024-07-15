use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    Overflow,
    MaxVotesReached,
    ProposalExpired,
    ProposalNotExpired,
    VoteLimitReached,
    ProposalStillActive,
    ArithmeticOverflow
}
