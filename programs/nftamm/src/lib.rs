//! An implementation of the SudoSwap AMM for the Solana blockchain.
#![warn(missing_docs)]
#![allow(rustdoc::missing_doc_code_examples)]

use anchor_lang::prelude::*;

mod error;
mod instructions;
pub mod state;
mod utils;

use instructions::*;

declare_id!("nftm2GmXMWeH8VCYxx8bAp3wL3tSx4Rp4LfZ1fmg6gk");

/// The nftamm program
#[program]
pub mod nftamm {
    use super::*;

    /// Initialize a new [state::PairAuthority]
    pub fn initialize_pair_authority(
        ctx: Context<InitializePairAuthority>,
        fees: u16,
    ) -> Result<()> {
        instructions::initialize_pair_authority::handler(ctx, fees)
    }

    /// Set the pending_authority of a [state::PairAuthority]
    pub fn transfer_pair_authority(ctx: Context<TransferPairAuthority>) -> Result<()> {
        instructions::transfer_pair_authority::handler(ctx)
    }

    /// Set the current_authority of a [state::PairAuthority] to it's pending_authority
    pub fn accept_pair_authority(ctx: Context<AcceptPairAuthority>) -> Result<()> {
        instructions::accept_pair_authority::handler(ctx)
    }

    /// Initialize a new [state::Pair]
    pub fn initialize_pair(
        ctx: Context<InitializePair>,
        pair_type: u8,
        bonding_curve: u8,
        delta: u64,
        fee: u16,
        spot_price: u64,
        honor_royalties: bool,
    ) -> Result<()> {
        instructions::initialize_pair::handler(
            ctx,
            pair_type,
            bonding_curve,
            delta,
            fee,
            spot_price,
            honor_royalties,
        )
    }

    /// Fund a token [state::Pair]
    pub fn fund_token_pair(ctx: Context<FundTokenPair>, amount_to_send: u64) -> Result<()> {
        instructions::fund_token_pair::handler(ctx, amount_to_send)
    }

    /// Fund a nft [state::Pair]
    pub fn fund_nft_pair(ctx: Context<FundNftPair>) -> Result<()> {
        instructions::fund_nft_pair::handler(ctx)
    }

    pub fn trade_token_pair(ctx: Context<TradeTokenPair>) -> Result<()> {
        instructions::trade_token_pair::handler(ctx)
    }

    pub fn trade_nft_pair(ctx: Context<TradeNftPair>) -> Result<()> {
        instructions::trade_nft_pair::handler(ctx)
    }

    pub fn swap_token_trade_pair(ctx: Context<SwapTokenTradePair>) -> Result<()> {
        instructions::swap_token_trade_pair::handler(ctx)
    }

    pub fn swap_nft_trade_pair(ctx: Context<SwapNftTradePair>) -> Result<()> {
        instructions::swap_nft_trade_pair::handler(ctx)
    }

    /// Set the delta of a [state::Pair] to new_delta
    pub fn change_delta(ctx: Context<ChangeDelta>, new_delta: u64) -> Result<()> {
        instructions::change_delta::handler(ctx, new_delta)
    }

    /// Set the fee of a [state::Pair] to new_fee
    pub fn change_fee(ctx: Context<ChangeFee>, new_fee: u16) -> Result<()> {
        instructions::change_fee::handler(ctx, new_fee)
    }

    /// Set the spot_price of a [state::Pair] to new_spot_price
    pub fn change_spot_price(ctx: Context<ChangeSpotPrice>, new_spot_price: u64) -> Result<()> {
        instructions::change_spot_price::handler(ctx, new_spot_price)
    }

    /// Close a token_pair
    pub fn close_pair(ctx: Context<ClosePair>) -> Result<()> {
        instructions::close_pair::handler(ctx)
    }

    /// Withdraw an nft from a [state::PairMetadata] and close the account - Pair creator only
    pub fn withdraw_nft(ctx: Context<WithdrawNft>) -> Result<()> {
        instructions::withdraw_nft::handler(ctx)
    }

    /// Withdraw a token from a [state::Pair]'s token_account - Pair creator only
    pub fn withdraw_quote_token(
        ctx: Context<WithdrawQuoteToken>,
        amount_to_withdraw: u64,
    ) -> Result<()> {
        instructions::withdraw_quote_token::handler(ctx, amount_to_withdraw)
    }

    /// Withdraw a token from a [state::Pair]'s fee account- Pair creator only
    pub fn withdraw_fee(ctx: Context<WithdrawFee>, amount: u64) -> Result<()> {
        instructions::withdraw_fee::handler(ctx, amount)
    }
}
