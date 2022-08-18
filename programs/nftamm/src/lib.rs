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
}
