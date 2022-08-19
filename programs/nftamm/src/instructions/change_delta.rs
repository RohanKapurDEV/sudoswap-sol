use crate::{error::ProgramError, state::Pair};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangeDelta<'info> {
    #[account(
        constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner,
    )]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub pair: Account<'info, Pair>,
}

pub fn handler(ctx: Context<ChangeDelta>, new_delta: u64) -> Result<()> {
    let pair = &mut ctx.accounts.pair;

    if pair.bonding_curve == 1 {
        if new_delta > 10000 {
            return Err(ProgramError::InvalidDelta.into());
        }
    }

    pair.delta = new_delta;

    Ok(())
}
