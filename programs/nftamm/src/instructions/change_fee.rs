use crate::{error::ProgramError, state::Pair};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangeFee<'info> {
    #[account(
        constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner,
    )]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub pair: Account<'info, Pair>,
}

pub fn handler(ctx: Context<ChangeFee>, new_fee: u16) -> Result<()> {
    let pair = &mut ctx.accounts.pair;

    if new_fee > 10000 {
        return Err(ProgramError::InvalidDelta.into());
    }

    pair.fee = new_fee;

    Ok(())
}
