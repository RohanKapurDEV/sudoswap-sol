use crate::{error::ProgramError, state::PairAuthority};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializePairAuthority<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<PairAuthority>(),
    )]
    pub pair_authority: Account<'info, PairAuthority>,

    pub system_program: Program<'info, System>,
}

/// Handler to initialize a pair authority.
pub fn handler(ctx: Context<InitializePairAuthority>, fees: u16) -> Result<()> {
    // Enforce basis points convention for fees
    if fees > 10000 {
        return Err(ProgramError::InvalidFees.into());
    }

    let pair_authority = &mut ctx.accounts.pair_authority;

    pair_authority.current_authority = ctx.accounts.payer.key();
    pair_authority.pending_authority = Pubkey::default();
    pair_authority.fees = fees;

    Ok(())
}
