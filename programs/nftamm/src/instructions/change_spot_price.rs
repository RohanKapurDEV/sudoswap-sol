use crate::{error::ProgramError, state::Pair};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangeSpotPrice<'info> {
    #[account(
        constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner,
    )]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub pair: Account<'info, Pair>,
}

pub fn handler(ctx: Context<ChangeSpotPrice>, new_spot_price: u64) -> Result<()> {
    let pair = &mut ctx.accounts.pair;

    pair.spot_price = new_spot_price;

    Ok(())
}
