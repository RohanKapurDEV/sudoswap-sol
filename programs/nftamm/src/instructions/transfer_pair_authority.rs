use crate::{error::ProgramError, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferPairAuthority<'info> {
    #[account(
        constraint = payer.key() == pair_authority.current_authority @ ProgramError::InvalidCurrentAuthority,
    )]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub pair_authority: Account<'info, PairAuthority>,

    /// CHECK: used as field for pair_authority
    pub pending_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferPairAuthority>) -> Result<()> {
    let pair_authority = &mut ctx.accounts.pair_authority;
    pair_authority.pending_authority = ctx.accounts.payer.key();

    Ok(())
}
