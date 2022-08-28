use crate::{
    error::ProgramError,
    state::{Pair, PairMetadata},
    utils::*,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct WithdrawNft<'info> {
    #[account(
        mut,
        constraint = payer.key() == pair.owner @ ProgramError::InvalidOwner,
    )]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub pair: Account<'info, Pair>,

    #[account(
        mut,
        constraint = pair_metadata_creator.key() == pair_metadata.creator @ ProgramError::InvalidCreator,
    )]
    pub pair_metadata_creator: UncheckedAccount<'info>,

    #[account(
        mut,
        close = pair_metadata_creator,
        seeds = [b"pair_metadata", pair.key().as_ref(), nft_token_mint.key().as_ref()],
        bump
    )]
    pub pair_metadata: Account<'info, PairMetadata>,

    #[account(constraint = nft_collection_mint.key() == pair.collection_mint @ ProgramError::InvalidCollectionMint)]
    pub nft_collection_mint: Box<Account<'info, Mint>>,

    /// CHECK: validated in access control logic
    pub nft_collection_metadata: UncheckedAccount<'info>,

    pub nft_token_mint: Box<Account<'info, Mint>>,

    /// CHECK: validated in access control logic
    pub nft_token_metadata: UncheckedAccount<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = nft_token_mint,
        associated_token::authority = payer
    )]
    pub owner_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = nft_token_vault.key() == pair_metadata.token_account @ ProgramError::InvalidNftTokenVault,
        constraint = nft_token_vault.mint == nft_token_mint.key() @ ProgramError::InvalidMint,
        constraint = nft_token_vault.owner == program_as_signer.key() @ ProgramError::InvalidMint,
    )]
    pub nft_token_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: PDA used as token account authority only
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> WithdrawNft<'info> {
    fn accounts(ctx: &Context<WithdrawNft>) -> Result<()> {
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

#[access_control(WithdrawNft::accounts(&ctx))]
pub fn handler(ctx: Context<WithdrawNft>) -> Result<()> {
    let pair = &mut ctx.accounts.pair;
    let program_as_signer_bump = *ctx.bumps.get("program_as_signer").unwrap();

    let transfer_nft_accounts = Transfer {
        from: ctx.accounts.nft_token_vault.to_account_info(),
        to: ctx.accounts.owner_nft_token_account.to_account_info(),
        authority: ctx.accounts.program_as_signer.to_account_info(),
    };

    let seeds = &[
        "program".as_bytes(),
        "signer".as_bytes(),
        &[program_as_signer_bump],
    ];

    let signer = &[&seeds[..]];

    let transfer_nft_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_nft_accounts,
        signer,
    );

    transfer(transfer_nft_ctx, 1)?;

    pair.nfts_held = pair.nfts_held.checked_sub(1).unwrap();

    if pair.pair_type == 1 {
        if pair.nfts_held == 0 {
            pair.is_active = false;
        }
    }

    Ok(())
}
