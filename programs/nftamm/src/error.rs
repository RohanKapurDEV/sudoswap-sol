use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramError {
    #[msg("Metadata account does not exist")]
    MetadataDoesntExist,
    #[msg("Edition account does not exist")]
    EditionDoesntExist,
    #[msg("Derived key invalid")]
    DerivedKeyInvalid,
    #[msg("Invalid pair type")]
    InvalidPairType,
    #[msg("Invalid bonding curve")]
    InvalidBondingCurve,
    #[msg("Invalid delta")]
    MetadataDeserializeError,
    #[msg("Invalid collection")]
    InvalidCollection,
    #[msg("Invalid collection mints")]
    InvalidCollectionMint,
    #[msg("Invalid collection details")]
    InvalidCollectionDetails,
    #[msg("Invalid funding amount")]
    InvalidFundingAmount,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Invalid quote token mint")]
    InvalidQuoteTokenMint,
    #[msg("Nft is not part of a verified collection")]
    NftNotVerified,
    #[msg("Pair is not active")]
    PairNotActive,
    #[msg("fee param must be between 0 and 10000")]
    InvalidFee,
    #[msg("delta param must be between 0 and 10000 for bonding curve 1")]
    InvalidDelta,
    #[msg("Fee too large")]
    FeeTooLarge,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Invalid quote token vault key")]
    InvalidQuoteTokenVault,
    #[msg("This token account does not have sufficient balance")]
    InsufficientBalance,
    #[msg("Invalid fee vault")]
    InvalidFeeVault,
    #[msg("Fee must be between 0 and 10000")]
    InvalidFees,
    #[msg("Cannot close pair account because it still holds nfts. Please withdraw all nfts first")]
    StillHoldsNfts,
    #[msg("Invalid pair authority")]
    InvalidPairAuthority,
    #[msg("Invalid current authority for pair authority")]
    InvalidCurrentAuthority,
    #[msg("Invalid pending authority for pair authority")]
    InvalidPendingAuthority,
    #[msg("Invalid nft token vault")]
    InvalidNftTokenVault,
    #[msg("Invalid creator for pair metadata")]
    InvalidCreator,
}
