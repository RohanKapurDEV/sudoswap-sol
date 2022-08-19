use crate::{error::ProgramError, state::Pair, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct TradeTokenPair<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, constraint = pair.pair_type == 0)]
    pub pair: Account<'info, Pair>,

    #[account(constraint = nft_collection_mint.key() == pair.collection_mint)]
    pub nft_collection_mint: Box<Account<'info, Mint>>,

    /// CHECK: validated in access control logic
    pub nft_collection_metadata: UncheckedAccount<'info>,

    pub nft_token_mint: Box<Account<'info, Mint>>,

    /// CHECK: validated in access control logic
    pub nft_token_metadata: UncheckedAccount<'info>,

    #[account(
        init,
        payer = payer,
        token::mint = nft_token_mint,
        token::authority = program_as_signer,
        seeds = [b"nft_account", pair.key().as_ref(), nft_token_mint.key().as_ref()],
        bump
    )]
    pub nft_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = user_nft_token_account.owner == payer.key(),
        constraint = user_nft_token_account.mint == nft_token_mint.key(),
        constraint = user_nft_token_account.amount == 1
    )]
    pub user_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(constraint = quote_token_mint.key() == pair.quote_token_mint)]
    pub quote_token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        constraint = quote_token_vault.key() == pair.quote_token_vault,
        constraint = quote_token_vault.mint == quote_token_mint.key(),
        constraint = quote_token_vault.amount == pair.spot_price
    )]
    pub quote_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = quote_token_mint,
        associated_token::authority = payer,
    )]
    pub user_quote_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: PDA used as token account authority only
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> TradeTokenPair<'info> {
    fn accounts(ctx: &Context<TradeTokenPair>) -> Result<()> {
        let pair = ctx.accounts.pair.clone();

        if !pair.is_active {
            return Err(ProgramError::PairNotActive.into());
        }

        let nft_token_mint = ctx.accounts.nft_token_mint.clone();
        let nft_token_metadata = ctx.accounts.nft_token_metadata.clone();

        let collection_mint = ctx.accounts.nft_collection_mint.clone();
        let collection_metadata = ctx.accounts.nft_collection_metadata.clone();

        validate_nft(
            *nft_token_mint,
            nft_token_metadata,
            *collection_mint,
            collection_metadata,
        )?;

        Ok(())
    }
}

#[access_control(TradeTokenPair::accounts(&ctx))]
pub fn handler(ctx: Context<TradeTokenPair>) -> Result<()> {
    let pair = &mut ctx.accounts.pair;
    let program_as_signer_bump = *ctx.bumps.get("program_as_signer").unwrap();

    let current_spot_price = pair.spot_price;

    // Transfer NFT from user to pair vault
    let transfer_nft_accounts = Transfer {
        from: ctx.accounts.user_nft_token_account.to_account_info(),
        to: ctx.accounts.nft_token_vault.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let transfer_nft_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_nft_accounts,
    );

    transfer(transfer_nft_ctx, 1)?;

    // Transfer quote_token from vault to user
    let transfer_quote_accounts = Transfer {
        from: ctx.accounts.quote_token_vault.to_account_info(),
        to: ctx.accounts.user_quote_token_account.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let seeds = &[
        "program".as_bytes(),
        "signer".as_bytes(),
        &[program_as_signer_bump],
    ];

    let signer = &[&seeds[..]];

    let transfer_quote_ctx = CpiContext::new_with_signer(
        ctx.accounts.associated_token_program.to_account_info(),
        transfer_quote_accounts,
        signer,
    );

    transfer(transfer_quote_ctx, current_spot_price)?;

    let bonding_curve = pair.bonding_curve;

    if bonding_curve == 0 {
        let delta = pair.delta;

        let new_spot_price = current_spot_price.checked_sub(delta).unwrap();
        pair.spot_price = new_spot_price;
    } else {
        let delta = pair.delta;

        // this is a very naive calculation, fix it later
        let new_spot_price = current_spot_price
            .checked_div(delta.checked_div(10000).unwrap().checked_add(1).unwrap())
            .unwrap();

        pair.spot_price = new_spot_price;
    }

    pair.nfts_held = pair.nfts_held.checked_add(1).unwrap();
    pair.trade_count = pair.trade_count.checked_add(1).unwrap();

    let quote_token_vault = &mut ctx.accounts.quote_token_vault;

    // If token pair can no longer sell quote tokens for
    if quote_token_vault.amount < current_spot_price.checked_sub(pair.delta).unwrap() {
        pair.is_active = false;
    }

    Ok(())
}
