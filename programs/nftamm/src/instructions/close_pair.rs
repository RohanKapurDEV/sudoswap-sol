use crate::error::ProgramError;
use crate::state::Pair;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClosePair<'info> {
    pub payer: Signer<'info>,

    #[account(
        mut,
        close = pair_owner,
    )]
    pub pair: Account<'info, Pair>,

    /// CHECK: only used as close target for pair_metadata
    #[account(
        mut,
        constraint = pair_owner.key() == pair.owner @ ProgramError::InvalidOwner,
    )]
    pub pair_owner: UncheckedAccount<'info>,
}

impl<'info> ClosePair<'info> {
    fn accounts(ctx: &Context<ClosePair>) -> Result<()> {
        let pair = ctx.accounts.pair.clone();

        if pair.nfts_held > 0 {
            return Err(ProgramError::StillHoldsNfts.into());
        }

        Ok(())
    }
}

#[access_control(ClosePair::accounts(&ctx))]
pub fn handler(ctx: Context<ClosePair>) -> Result<()> {
    Ok(())
}
