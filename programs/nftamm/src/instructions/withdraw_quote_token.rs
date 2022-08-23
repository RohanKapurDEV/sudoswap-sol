use crate::{error::ProgramError, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct WithdrawQuoteToken<'info> {
    #[account(
        mut,
        constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner,
    )]
    pub payer: Signer<'info>,

    #[account(constraint = pair_authority.key() == pair.pair_authority @ ProgramError::InvalidPairAuthority)]
    pub pair_authority: Account<'info, PairAuthority>,

    #[account(mut)]
    pub pair: Account<'info, Pair>,

    #[account(constraint = quote_token_mint.key() == pair.quote_token_mint @ ProgramError::InvalidQuoteTokenMint)]
    pub quote_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = quote_token_vault.mint == quote_token_mint.key() @ ProgramError::InvalidMint,
        constraint = quote_token_vault.owner == program_as_signer.key() @ ProgramError::InvalidOwner,
        constraint = quote_token_vault.key() == pair.quote_token_vault @ ProgramError::InvalidQuoteTokenVault,
    )]
    pub quote_token_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = quote_token_mint,
        associated_token::authority = payer
    )]
    pub owner_quote_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA used as token account authority only
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> WithdrawQuoteToken<'info> {
    fn accounts(ctx: &Context<WithdrawQuoteToken>, amount_to_withdraw: u64) -> Result<()> {
        let quote_token_vault = ctx.accounts.quote_token_vault.clone();

        if quote_token_vault.amount < amount_to_withdraw {
            return Err(ProgramError::InsufficientBalance.into());
        }

        Ok(())
    }
}

#[access_control(WithdrawQuoteToken::accounts(&ctx, amount_to_withdraw))]
pub fn handler(ctx: Context<WithdrawQuoteToken>, amount_to_withdraw: u64) -> Result<()> {
    let pair = &mut ctx.accounts.pair;
    let pair_authority = &mut ctx.accounts.pair_authority;
    let program_as_signer_bump = *ctx.bumps.get("program_as_signer").unwrap();
    let quote_token_vault = &mut ctx.accounts.quote_token_vault;

    let pair_authority_fees = pair_authority.fees;

    let pair_auth_fee_applied = pair
        .spot_price
        .checked_mul(pair_authority_fees as u64)
        .unwrap()
        .checked_div(10000)
        .unwrap();

    let current_spot_price = pair.spot_price;

    let transfer_quote_token_accounts = Transfer {
        from: quote_token_vault.to_account_info(),
        to: ctx.accounts.owner_quote_token_account.to_account_info(),
        authority: ctx.accounts.program_as_signer.to_account_info(),
    };

    let seeds = &[
        "program".as_bytes(),
        "signer".as_bytes(),
        &[program_as_signer_bump],
    ];

    let signer = &[&seeds[..]];

    let transfer_quote_token_accounts_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_quote_token_accounts,
        signer,
    );

    transfer(transfer_quote_token_accounts_ctx, amount_to_withdraw)?;

    if pair.pair_type == 0 {
        if quote_token_vault.amount
            < current_spot_price
                .checked_add(pair_auth_fee_applied)
                .unwrap()
        {
            pair.is_active = false;
        }
    }

    Ok(())
}
