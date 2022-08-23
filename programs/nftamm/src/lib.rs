use anchor_lang::prelude::*;

mod error;
mod instructions;
mod state;
mod utils;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nftamm {
    use super::*;

    pub fn initialize_pair_authority(
        ctx: Context<InitializePairAuthority>,
        fees: u16,
    ) -> Result<()> {
        instructions::initialize_pair_authority::handler(ctx, fees)
    }

    pub fn transfer_pair_authority(ctx: Context<TransferPairAuthority>) -> Result<()> {
        instructions::transfer_pair_authority::handler(ctx)
    }

    pub fn accept_pair_authority(ctx: Context<AcceptPairAuthority>) -> Result<()> {
        instructions::accept_pair_authority::handler(ctx)
    }

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

    pub fn fund_token_pair(ctx: Context<FundTokenPair>, amount_to_send: u64) -> Result<()> {
        instructions::fund_token_pair::handler(ctx, amount_to_send)
    }

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

    pub fn change_delta(ctx: Context<ChangeDelta>, new_delta: u64) -> Result<()> {
        instructions::change_delta::handler(ctx, new_delta)
    }

    pub fn change_fee(ctx: Context<ChangeFee>, new_fee: u16) -> Result<()> {
        instructions::change_fee::handler(ctx, new_fee)
    }

    pub fn change_spot_price(ctx: Context<ChangeSpotPrice>, new_spot_price: u64) -> Result<()> {
        instructions::change_spot_price::handler(ctx, new_spot_price)
    }

    pub fn close_pair(ctx: Context<ClosePair>) -> Result<()> {
        instructions::close_pair::handler(ctx)
    }

    pub fn withdraw_nft(ctx: Context<WithdrawNft>) -> Result<()> {
        instructions::withdraw_nft::handler(ctx)
    }
}
