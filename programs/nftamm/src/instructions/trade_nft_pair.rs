use crate::{error::ProgramError, state::Pair, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct TradeNftPair<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, constraint = pair.pair_type == 1)]
    pub pair: Account<'info, Pair>,

    #[account(constraint = nft_collection_mint.key() == pair.collection_mint)]
    pub nft_collection_mint: Account<'info, Mint>,

    /// CHECK: validated in access control logic
    pub nft_collection_metadata: UncheckedAccount<'info>,

    pub nft_token_mint: Account<'info, Mint>,

    /// CHECK: validated in access control logic
    pub nft_token_metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"nft_account", pair.key().as_ref(), nft_token_mint.key().as_ref()],
        bump
    )]
    pub nft_token_vault: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = nft_token_mint,
        associated_token::authority = payer,
    )]
    pub user_nft_token_account: Account<'info, TokenAccount>,

    #[account(constraint = quote_token_mint.key() == pair.quote_token_mint)]
    pub quote_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"quote", pair.key().as_ref()],
        bump,
        constraint = quote_token_vault.key() == pair.quote_token_vault,
        constraint = quote_token_vault.mint == quote_token_mint.key(),
    )]
    pub quote_token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_quote_token_account.mint == quote_token_mint.key(),
        constraint = user_quote_token_account.owner == payer.key(),
    )]
    pub user_quote_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> TradeNftPair<'info> {
    fn accounts(ctx: &Context<TradeNftPair>) -> Result<()> {
        let pair = ctx.accounts.pair.clone();

        if !pair.is_active {
            return Err(ProgramError::PairNotActive.into());
        }

        let nft_token_mint = ctx.accounts.nft_token_mint.clone();
        let nft_token_metadata = ctx.accounts.nft_token_metadata.clone();

        let collection_mint = ctx.accounts.nft_collection_mint.clone();
        let collection_metadata = ctx.accounts.nft_collection_metadata.clone();

        validate_nft(
            nft_token_mint,
            nft_token_metadata,
            collection_mint,
            collection_metadata,
        )?;

        Ok(())
    }
}

pub fn handler(ctx: Context<TradeNftPair>) -> Result<()> {
    Ok(())
}
