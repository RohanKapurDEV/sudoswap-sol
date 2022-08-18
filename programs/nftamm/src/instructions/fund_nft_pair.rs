use crate::{error::ProgramError, state::Pair, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct FundNftPair<'info> {
    #[account(mut, constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner)]
    pub payer: Signer<'info>,

    #[account(mut)]
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
        constraint = owner_nft_token_account.mint == nft_token_mint.key()
    )]
    pub owner_nft_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        token::mint = nft_token_mint,
        token::authority = program_as_signer,
        seeds = [b"nft_account", pair.key().as_ref(), nft_token_mint.key().as_ref()],
        bump
    )]
    pub nft_token_vault: Account<'info, TokenAccount>,

    /// CHECK: PDA used as token account authority only
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> FundNftPair<'info> {
    fn accounts(ctx: &Context<FundNftPair>) -> Result<()> {
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

#[access_control(FundNftPair::accounts(&ctx))]
pub fn handler(ctx: Context<FundNftPair>) -> Result<()> {
    let pair = &mut ctx.accounts.pair;

    // This can be called on nft pairs or trade pairs
    if pair.pair_type != 1 || pair.pair_type != 2 {
        return Err(ProgramError::InvalidPairType.into());
    }

    let transfer_accounts = Transfer {
        from: ctx.accounts.owner_nft_token_account.to_account_info(),
        to: ctx.accounts.nft_token_vault.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
    };

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    transfer(transfer_ctx, 1)?;

    if pair.is_active == false {
        pair.is_active = true;
    }

    let current_held = pair.nfts_held;
    pair.nfts_held = current_held.checked_add(1).unwrap();

    Ok(())
}
