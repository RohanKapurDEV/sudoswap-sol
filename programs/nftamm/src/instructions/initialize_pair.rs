use crate::error::ProgramError;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

use crate::{state::Pair, utils::*};

#[derive(Accounts)]
pub struct InitializePair<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Pair::SIZE
    )]
    pub pair: Account<'info, Pair>,

    /// CHECK: validated in access control logic
    pub nft_collection_mint: Account<'info, Mint>,
    /// CHECK: validated in access control logic
    pub nft_collection_metadata: UncheckedAccount<'info>,

    pub quote_token_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = payer,
        token::mint = quote_token_mint,
        token::authority = program_as_signer,
        seeds = [b"quote", pair.key().as_ref()],
        bump
    )]
    pub quote_token_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = payer,
        token::mint = quote_token_mint,
        token::authority = program_as_signer,
        seeds = [b"quote", "fee".as_bytes(), pair.key().as_ref()],
        bump
    )]
    pub fee_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,

    /// CHECK: PDA used as token account authority only
    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,
}

impl<'info> InitializePair<'info> {
    fn accounts(ctx: &Context<InitializePair>, pair_type: u8, bonding_curve: u8) -> Result<()> {
        // Validate that collection metadata exists
        let collection_mint = ctx.accounts.nft_collection_mint.clone();
        let collection_metadata = ctx.accounts.nft_collection_metadata.clone();

        assert_metadata_valid(&collection_metadata, &collection_mint.key())?;

        // Check to see that Collection and CollectionDetails fields on metadata account are valid
        let metadata: Metadata =
            Metadata::from_account_info(&ctx.accounts.nft_collection_metadata.to_account_info())?;

        let collection = metadata.collection;
        let collection_details = metadata.collection_details;

        // metadata.collection must be None
        if collection.is_some() {
            return Err(ProgramError::InvalidCollection.into());
        }

        // metadata.collection_details must be Some(V1 {...})
        if collection_details.is_none() {
            return Err(ProgramError::InvalidCollectionDetails.into());
        }

        // Validate pair type and bonding curve
        if pair_type > 2 {
            return Err(ProgramError::InvalidPairType.into());
        }

        if bonding_curve > 1 {
            return Err(ProgramError::InvalidBondingCurve.into());
        }

        Ok(())
    }
}

#[access_control(InitializePair::accounts(&ctx, pair_type, bonding_curve))]
pub fn handler(
    ctx: Context<InitializePair>,
    pair_type: u8,
    bonding_curve: u8,
    delta: u64,
    fee: u16,
    spot_price: u64,
    honor_royalties: bool,
) -> Result<()> {
    if fee > 10000 {
        return Err(ProgramError::InvalidFee.into());
    }

    let pair = &mut ctx.accounts.pair;

    pair.owner = ctx.accounts.payer.key();
    pair.collection_mint = ctx.accounts.nft_collection_mint.key();
    pair.quote_token_mint = ctx.accounts.quote_token_mint.key();
    pair.quote_token_vault = ctx.accounts.quote_token_vault.key();
    pair.pair_type = pair_type;
    pair.bonding_curve = bonding_curve;
    pair.delta = delta;
    pair.fee = fee;
    pair.fee_vault = ctx.accounts.fee_vault.key();
    pair.spot_price = spot_price;
    pair.honor_royalties = honor_royalties;
    pair.trade_count = 0;
    pair.is_active = false;
    pair.nfts_held = 0;

    Ok(())
}
