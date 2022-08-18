use crate::error::ProgramError;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use mpl_token_metadata::state::{Metadata, TokenMetadataAccount};

// Function taken from auction house contract
pub fn assert_metadata_valid<'a>(metadata: &UncheckedAccount, mint: &Pubkey) -> Result<()> {
    assert_derivation(
        &mpl_token_metadata::id(),
        &metadata.to_account_info(),
        &[
            mpl_token_metadata::state::PREFIX.as_bytes(),
            mpl_token_metadata::id().as_ref(),
            mint.as_ref(),
        ],
    )?;
    if metadata.data_is_empty() {
        return Err(ProgramError::MetadataDoesntExist.into());
    }

    Ok(())
}

// Function taken from auction house contract
pub fn assert_derivation(program_id: &Pubkey, account: &AccountInfo, path: &[&[u8]]) -> Result<u8> {
    let (key, bump) = Pubkey::find_program_address(&path, program_id);
    if key != *account.key {
        return Err(ProgramError::DerivedKeyInvalid.into());
    }
    Ok(bump)
}

// Custom function to validate nft belongs to a specified collection and is verified
pub fn validate_nft(
    nft_token_mint: Account<Mint>,
    nft_token_metadata: UncheckedAccount,
    collection_mint: Account<Mint>,
    collection_metadata: UncheckedAccount,
) -> Result<()> {
    assert_metadata_valid(&nft_token_metadata, &nft_token_mint.key())?;

    let token_metadata: Metadata =
        Metadata::from_account_info(&nft_token_metadata.to_account_info())?;

    let token_collection = token_metadata.collection;
    let token_collection_details = token_metadata.collection_details;

    if token_collection_details.is_some() {
        return Err(ProgramError::InvalidCollectionDetails.into());
    }

    if token_collection.is_none() {
        return Err(ProgramError::InvalidCollection.into());
    }

    if token_collection.clone().unwrap().key != collection_mint.key() {
        return Err(ProgramError::InvalidCollectionMint.into());
    }

    if token_collection.unwrap().verified != true {
        return Err(ProgramError::NftNotVerified.into());
    }

    assert_metadata_valid(&collection_metadata, &collection_mint.key())?;

    let collection_metadata: Metadata =
        Metadata::from_account_info(&collection_metadata.to_account_info())?;

    let collection_collection = collection_metadata.collection;
    let collection_collection_details = collection_metadata.collection_details;

    if collection_collection.is_some() {
        return Err(ProgramError::InvalidCollection.into());
    }

    if collection_collection_details.is_none() {
        return Err(ProgramError::InvalidCollectionDetails.into());
    }

    Ok(())
}
