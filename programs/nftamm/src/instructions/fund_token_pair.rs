use crate::{error::ProgramError, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct FundTokenPair<'info> {
    #[account(constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner)]
    pub payer: Signer<'info>,

    #[account(
        constraint = pair_authority.key() == pair.pair_authority @ ProgramError::InvalidPairAuthority,
    )]
    pub pair_authority: Account<'info, PairAuthority>,

    #[account(mut)]
    pub pair: Account<'info, Pair>,

    #[account(constraint = quote_token_mint.key() == pair.quote_token_mint @ ProgramError::InvalidQuoteTokenMint)]
    pub quote_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"quote", pair.key().as_ref()],
        bump,
        constraint = quote_token_vault.mint == quote_token_mint.key() @ ProgramError::InvalidMint,
        constraint = quote_token_vault.owner == program_as_signer.key() @ ProgramError::InvalidOwner,
        constraint = quote_token_vault.key() == pair.quote_token_vault @ ProgramError::InvalidQuoteTokenVault,
    )]
    pub quote_token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = owner_quote_token_account.owner == payer.key() @ ProgramError::InvalidOwner,
        constraint = owner_quote_token_account.mint == quote_token_mint.key() @ ProgramError::InvalidQuoteTokenMint,
    )]
    pub owner_quote_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA used as token account authority only
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<FundTokenPair>, amount_to_send: u64) -> Result<()> {
    let pair = &mut ctx.accounts.pair;
    let pair_authority = &mut ctx.accounts.pair_authority;

    let pair_authority_fees = pair_authority.fees;

    let pair_auth_fee_applied = pair
        .spot_price
        .checked_mul(pair_authority_fees as u64)
        .unwrap()
        .checked_div(10000)
        .unwrap();

    let current_spot_price = pair.spot_price;

    // This can be called on token pairs or trade pairs
    if pair.pair_type != 0 || pair.pair_type != 2 {
        return Err(ProgramError::InvalidPairType.into());
    }

    let transfer_accounts = Transfer {
        from: ctx.accounts.owner_quote_token_account.to_account_info(),
        to: ctx.accounts.quote_token_vault.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    transfer(transfer_ctx, amount_to_send)?;

    if pair.pair_type == 0 {
        if ctx.accounts.quote_token_vault.amount
            < current_spot_price
                .checked_add(pair_auth_fee_applied)
                .unwrap()
        {
            pair.is_active = false;
        }
    }

    Ok(())
}
