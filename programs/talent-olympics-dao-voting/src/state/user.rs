use anchor_lang::prelude::*;

use crate::error::CustomError;

#[account]
#[derive(InitialSize)]
pub struct User {
    pub score: u64,
}

impl User {
    pub fn increment_points(&mut self, points: u64) -> Result<()> {
        self.score = self
            .score
            .checked_add(points)
            .ok_or(CustomError::ArithmeticOverflow)?;
        Ok(())
    }
}
