use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use crate::{error::ProgramError, state::Pair};

#[derive(Accounts)]
pub struct FundTokenPair<'info> {
    #[account(constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner)]
    pub payer: Signer<'info>,

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
        constraint = owner_quote_token_account.amount >= pair.spot_price @ ProgramError::InsufficientBalance,
    )]
    pub owner_quote_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA used as token account authority only
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<FundTokenPair>, amount_to_send: u64) -> Result<()> {
    let pair = &mut ctx.accounts.pair;

    // This can be called on token pairs or trade pairs
    if pair.pair_type != 0 || pair.pair_type != 2 {
        return Err(ProgramError::InvalidPairType.into());
    }

    if amount_to_send < pair.spot_price {
        return Err(ProgramError::InvalidFundingAmount.into());
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

    if pair.is_active == false {
        pair.is_active = true;
    }

    Ok(())
}
