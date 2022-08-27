use crate::{error::ProgramError, state::Pair};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct WithdrawFee<'info> {
    #[account(
        mut,
        constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner,
    )]
    pub payer: Signer<'info>,

    pub pair: Account<'info, Pair>,

    /// CHECK: only used as close target for pair_metadata
    #[account(
        mut,
        constraint = pair_owner.key() == pair.owner @ ProgramError::InvalidOwner,
    )]
    pub pair_owner: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = quote_token_mint,
        associated_token::authority = pair_owner
    )]
    pub pair_owner_quote_token_account: Account<'info, TokenAccount>,

    #[account(constraint = quote_token_mint.key() == pair.quote_token_mint)]
    pub quote_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [b"quote", "fee".as_bytes(), pair.key().as_ref()],
        bump,
        constraint = quote_fee_vault.key() == pair.fee_vault @ ProgramError::InvalidFeeVault ,
        constraint = quote_fee_vault.mint == quote_token_mint.key() @ ProgramError::InvalidQuoteTokenMint,
        constraint = quote_fee_vault.owner == program_as_signer.key() @ ProgramError::InvalidOwner,
    )]
    pub quote_fee_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: PDA used as token account authority only
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> WithdrawFee<'info> {
    fn accounts(ctx: &Context<WithdrawFee>, amount: u64) -> Result<()> {
        let fee_vault = ctx.accounts.quote_fee_vault.clone();

        if fee_vault.amount < amount {
            return Err(ProgramError::InsufficientBalance.into());
        }

        Ok(())
    }
}

#[access_control(WithdrawFee::accounts(&ctx, amount))]
pub fn handler(ctx: Context<WithdrawFee>, amount: u64) -> Result<()> {
    let program_as_signer_bump = *ctx.bumps.get("program_as_signer").unwrap();

    let transfer_fee_vault_accounts = Transfer {
        from: ctx.accounts.quote_fee_vault.to_account_info(),
        to: ctx
            .accounts
            .pair_owner_quote_token_account
            .to_account_info(),
        authority: ctx.accounts.program_as_signer.to_account_info(),
    };

    let seeds = &[
        "program".as_bytes(),
        "signer".as_bytes(),
        &[program_as_signer_bump],
    ];

    let signer = &[&seeds[..]];

    let transfer_fee_vault_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_fee_vault_accounts,
        signer,
    );

    transfer(transfer_fee_vault_ctx, amount)?;

    Ok(())
}
