use crate::{error::ProgramError, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AcceptPairAuthority<'info> {
    #[account(
        constraint = payer.key() == pair_authority.pending_authority @ ProgramError::InvalidPendingAuthority,
    )]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub pair_authority: Account<'info, PairAuthority>,
}

pub fn handler(ctx: Context<AcceptPairAuthority>) -> Result<()> {
    let pair_authority = &mut ctx.accounts.pair_authority;

    pair_authority.current_authority = ctx.accounts.payer.key();
    pair_authority.pending_authority = Pubkey::default();

    Ok(())
}
