use anchor_lang::prelude::*;

pub const HEADER_SIZE: usize = std::mem::size_of::<u64>();

#[constant]
pub const PROPOSAL_IDENTIFIER: &[u8] = b"prop";

#[constant]
pub const MEMBER_IDENTIFIER: &[u8] = b"user";
